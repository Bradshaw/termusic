// use std::io::{Read, Seek};
// use std::marker::Sync;
use std::sync::{Arc, Weak};
use std::{error, fmt};

use super::decoder;
use super::dynamic_mixer::{self, DynamicMixerController};
// use super::sink::Sink;
// use super::cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use super::cpal::Sample;
use super::source::Source;
use super::{default_host, DeviceTrait, HostTrait, StreamTrait};
use super::{CpalSample as Sample, Device, Stream};

/// `cpal::Stream` container. Also see the more useful `OutputStreamHandle`.
///
/// If this is dropped playback will end & attached `OutputStreamHandle`s will no longer work.
#[allow(clippy::module_name_repetitions)]
pub struct OutputStream {
    mixer: Arc<DynamicMixerController<f32>>,
    // _stream: super::cpal::Stream,
    _stream: Stream,
}

/// More flexible handle to a `OutputStream` that provides playback.
#[derive(Clone)]
pub struct OutputStreamHandle {
    mixer: Weak<DynamicMixerController<f32>>,
}

impl OutputStream {
    /// Returns a new stream & handle using the given output device.
    pub fn try_from_device(
        // device: &super::cpal::Device,
        device: &Device,
    ) -> Result<(Self, OutputStreamHandle), StreamError> {
        let (mixer, stream) = device.try_new_output_stream()?;
        stream.play()?;
        let out = Self {
            mixer,
            _stream: stream,
        };
        let handle = OutputStreamHandle {
            mixer: Arc::downgrade(&out.mixer),
        };
        Ok((out, handle))
    }

    /// Return a new stream & handle using the default output device.
    ///
    /// On failure will fallback to trying any non-default output devices.
    pub fn try_default() -> Result<(Self, OutputStreamHandle), StreamError> {
        // gag is not working
        // #[cfg(unix)]
        // let _gag = gag::Gag::stderr().unwrap();
        // eprintln!("gag works?");

        // let default_device = super::cpal::default_host()
        let default_device = default_host()
            .default_output_device()
            .ok_or(StreamError::NoDevice)?;

        let default_stream = Self::try_from_device(&default_device);

        default_stream.or_else(|original_err| {
            // default device didn't work, try other ones
            // let mut devices = match super::cpal::default_host().output_devices() {
            let mut devices = match default_host().output_devices() {
                Ok(d) => d,
                Err(_) => return Err(original_err),
            };

            devices
                .find_map(|d| Self::try_from_device(&d).ok())
                .ok_or(original_err)
        })
    }
}

#[allow(unused)]
impl OutputStreamHandle {
    /// Plays a source with a device until it ends.
    pub fn play_raw<S>(&self, source: S) -> Result<(), PlayError>
    where
        S: Source<Item = f32> + Send + 'static,
    {
        let mixer = self.mixer.upgrade().ok_or(PlayError::NoDevice)?;
        mixer.add(source);
        Ok(())
    }

    // Plays a sound once. Returns a `Sink` that can be used to control the sound.
    // pub fn play_once<R>(&self, input: R) -> Result<Sink, PlayError>
    // where
    //     R: Read + Seek + Send + Sync + 'static,
    // {
    //     let input = decoder::Decoder::new_decoder(input)?;
    //     let sink = Sink::try_new(self)?;
    //     sink.append(input);
    //     Ok(sink)
    // }
}

/// An error occurred while attemping to play a sound.
#[derive(Debug)]
pub enum PlayError {
    /// Attempting to decode the audio failed.
    DecoderError(decoder::SymphoniaDecoderError),
    /// The output device was lost.
    NoDevice,
}

impl From<decoder::SymphoniaDecoderError> for PlayError {
    fn from(err: decoder::SymphoniaDecoderError) -> Self {
        Self::DecoderError(err)
    }
}

impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecoderError(e) => e.fmt(f),
            Self::NoDevice => write!(f, "NoDevice"),
        }
    }
}

impl error::Error for PlayError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::DecoderError(e) => Some(e),
            Self::NoDevice => None,
        }
    }
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
pub enum StreamError {
    // PlayStreamError(super::cpal::PlayStreamError),
    // DefaultStreamConfigError(super::cpal::DefaultStreamConfigError),
    // BuildStreamError(super::cpal::BuildStreamError),
    // SupportedStreamConfigsError(super::cpal::SupportedStreamConfigsError),
    PlayStreamError(super::PlayStreamError),
    DefaultStreamConfigError(super::DefaultStreamConfigError),
    BuildStreamError(super::BuildStreamError),
    SupportedStreamConfigsError(super::SupportedStreamConfigsError),
    NoDevice,
}

impl From<super::DefaultStreamConfigError> for StreamError {
    fn from(err: super::DefaultStreamConfigError) -> Self {
        Self::DefaultStreamConfigError(err)
    }
}

impl From<super::SupportedStreamConfigsError> for StreamError {
    fn from(err: super::SupportedStreamConfigsError) -> Self {
        Self::SupportedStreamConfigsError(err)
    }
}

impl From<super::BuildStreamError> for StreamError {
    fn from(err: super::BuildStreamError) -> Self {
        Self::BuildStreamError(err)
    }
}

impl From<super::PlayStreamError> for StreamError {
    fn from(err: super::PlayStreamError) -> Self {
        Self::PlayStreamError(err)
    }
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlayStreamError(e) => e.fmt(f),
            Self::BuildStreamError(e) => e.fmt(f),
            Self::DefaultStreamConfigError(e) => e.fmt(f),
            Self::SupportedStreamConfigsError(e) => e.fmt(f),
            Self::NoDevice => write!(f, "NoDevice"),
        }
    }
}

impl error::Error for StreamError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::PlayStreamError(e) => Some(e),
            Self::BuildStreamError(e) => Some(e),
            Self::DefaultStreamConfigError(e) => Some(e),
            Self::SupportedStreamConfigsError(e) => Some(e),
            Self::NoDevice => None,
        }
    }
}

/// Extensions to `cpal::Device`
pub trait CpalDeviceExt {
    fn new_output_stream_with_format(
        &self,
        format: super::SupportedStreamConfig,
    ) -> Result<(Arc<DynamicMixerController<f32>>, Stream), super::BuildStreamError>;

    fn try_new_output_stream(
        &self,
    ) -> Result<(Arc<DynamicMixerController<f32>>, Stream), StreamError>;
}

impl CpalDeviceExt for Device {
    fn new_output_stream_with_format(
        &self,
        format: super::SupportedStreamConfig,
    ) -> Result<(Arc<DynamicMixerController<f32>>, Stream), super::BuildStreamError> {
        let (mixer_tx, mut mixer_rx) =
            dynamic_mixer::mixer::<f32>(format.channels(), format.sample_rate().0);

        let error_callback = |err| eprintln!("an error occurred on output stream: {err}");

        match format.sample_format() {
            super::SampleFormat::F32 => self.build_output_stream::<f32, _, _>(
                &format.config(),
                move |data, _| {
                    data.iter_mut()
                        .for_each(|d| *d = mixer_rx.next().unwrap_or(0_f32));
                },
                error_callback,
            ),
            super::SampleFormat::I16 => self.build_output_stream::<i16, _, _>(
                &format.config(),
                move |data, _| {
                    data.iter_mut()
                        .for_each(|d| *d = mixer_rx.next().map_or(0_i16, |s| s.to_i16()));
                },
                error_callback,
            ),
            super::SampleFormat::U16 => self.build_output_stream::<u16, _, _>(
                &format.config(),
                move |data, _| {
                    for d in data.iter_mut() {
                        *d = mixer_rx.next().map_or(u16::max_value() / 2, |s| s.to_u16());
                    }
                },
                error_callback,
            ),
        }
        .map(|stream| (mixer_tx, stream))
    }

    fn try_new_output_stream(
        &self,
    ) -> Result<(Arc<DynamicMixerController<f32>>, Stream), StreamError> {
        // Determine the format to use for the new stream.
        let default_format = self.default_output_config()?;

        self.new_output_stream_with_format(default_format)
            .or_else(|err| {
                // look through all supported formats to see if another works
                supported_output_formats(self)?
                    .find_map(|format| self.new_output_stream_with_format(format).ok())
                    // return original error if nothing works
                    .ok_or(StreamError::BuildStreamError(err))
            })
    }
}

/// All the supported output formats with sample rates
fn supported_output_formats(
    device: &Device,
) -> Result<impl Iterator<Item = super::SupportedStreamConfig>, StreamError> {
    const HZ_44100: super::SampleRate = super::SampleRate(44_100);

    let mut supported: Vec<_> = device.supported_output_configs()?.collect();
    supported.sort_by(|a, b| b.cmp_default_heuristics(a));

    Ok(supported.into_iter().flat_map(|sf| {
        let max_rate = sf.max_sample_rate();
        let min_rate = sf.min_sample_rate();
        let mut formats = vec![sf.clone().with_max_sample_rate()];
        if HZ_44100 < max_rate && HZ_44100 > min_rate {
            formats.push(sf.clone().with_sample_rate(HZ_44100));
        }
        formats.push(sf.with_sample_rate(min_rate));
        formats
    }))
}
