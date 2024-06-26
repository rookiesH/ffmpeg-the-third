use std::ops::{Deref, DerefMut};

#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::ffi::*;
#[cfg(not(feature = "ffmpeg_5_0"))]
use libc::c_int;

use super::Opened;
use crate::codec::Context;
#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::frame;
use crate::util::format;
use crate::AudioService;
#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::{packet, Error};

#[cfg(feature = "ffmpeg_5_1")]
use crate::ChannelLayout;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

pub struct Audio(pub Opened);

impl Audio {
    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_decode_audio4 has been deprecated since FFmpeg 3.1; \
        consider switching to send_packet() and receive_frame()"
    )]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn decode<P: packet::Ref>(
        &mut self,
        packet: &P,
        out: &mut frame::Audio,
    ) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_decode_audio4(
                self.as_mut_ptr(),
                out.as_mut_ptr(),
                &mut got,
                packet.as_ptr(),
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }

    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }

    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }

    pub fn request_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).request_sample_fmt = value.into();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn frames(&self) -> usize {
        unsafe { (*self.as_ptr()).frame_number as usize }
    }

    pub fn align(&self) -> usize {
        unsafe { (*self.as_ptr()).block_align as usize }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layout(&self) -> ChannelLayoutMask {
        unsafe { ChannelLayoutMask::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn request_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).request_channel_layout = value.bits();
        }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layout(&self) -> ChannelLayout {
        unsafe { ChannelLayout::from(&self.as_ptr().as_ref().unwrap().ch_layout) }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn set_ch_layout(&mut self, value: ChannelLayout) {
        unsafe {
            self.as_mut_ptr().as_mut().unwrap().ch_layout = value.into_owned();
        }
    }

    pub fn audio_service(&mut self) -> AudioService {
        unsafe { AudioService::from((*self.as_mut_ptr()).audio_service_type) }
    }

    pub fn max_bit_rate(&self) -> usize {
        unsafe { (*self.as_ptr()).rc_max_rate as usize }
    }

    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn frame_start(&self) -> Option<usize> {
        unsafe {
            // Removed in ffmpeg >= 5.0 in favor of using encoder
            // private options.
            match (*self.as_ptr()).timecode_frame_start {
                -1 => None,
                n => Some(n as usize),
            }
        }
    }
}

impl Deref for Audio {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Audio {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Audio {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
