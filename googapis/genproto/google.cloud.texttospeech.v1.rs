/// The top-level message sent by the client for the `ListVoices` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVoicesRequest {
    /// Optional. Recommended.
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag. If
    /// specified, the ListVoices call will only return voices that can be used to
    /// synthesize this language_code. E.g. when specifying `"en-NZ"`, you will get
    /// supported `"en-\*"` voices; when specifying `"no"`, you will get supported
    /// `"no-\*"` (Norwegian) and `"nb-\*"` (Norwegian Bokmal) voices; specifying
    /// `"zh"` will also get supported `"cmn-\*"` voices; specifying `"zh-hk"` will
    /// also get supported `"yue-\*"` voices.
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
}
/// The message returned to the client by the `ListVoices` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVoicesResponse {
    /// The list of voices.
    #[prost(message, repeated, tag = "1")]
    pub voices: ::prost::alloc::vec::Vec<Voice>,
}
/// Description of a voice supported by the TTS service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Voice {
    /// The languages that this voice supports, expressed as
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tags (e.g.
    /// "en-US", "es-419", "cmn-tw").
    #[prost(string, repeated, tag = "1")]
    pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The name of this voice.  Each distinct voice has a unique name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The gender of this voice.
    #[prost(enumeration = "SsmlVoiceGender", tag = "3")]
    pub ssml_gender: i32,
    /// The natural sample rate (in hertz) for this voice.
    #[prost(int32, tag = "4")]
    pub natural_sample_rate_hertz: i32,
}
/// The top-level message sent by the client for the `SynthesizeSpeech` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechRequest {
    /// Required. The Synthesizer requires either plain text or SSML as input.
    #[prost(message, optional, tag = "1")]
    pub input: ::core::option::Option<SynthesisInput>,
    /// Required. The desired voice of the synthesized audio.
    #[prost(message, optional, tag = "2")]
    pub voice: ::core::option::Option<VoiceSelectionParams>,
    /// Required. The configuration of the synthesized audio.
    #[prost(message, optional, tag = "3")]
    pub audio_config: ::core::option::Option<AudioConfig>,
}
/// Contains text input to be synthesized. Either `text` or `ssml` must be
/// supplied. Supplying both or neither returns
/// \[google.rpc.Code.INVALID_ARGUMENT][\]. The input size is limited to 5000
/// characters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesisInput {
    /// The input source, which is either plain text or SSML.
    #[prost(oneof = "synthesis_input::InputSource", tags = "1, 2")]
    pub input_source: ::core::option::Option<synthesis_input::InputSource>,
}
/// Nested message and enum types in `SynthesisInput`.
pub mod synthesis_input {
    /// The input source, which is either plain text or SSML.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InputSource {
        /// The raw text to be synthesized.
        #[prost(string, tag = "1")]
        Text(::prost::alloc::string::String),
        /// The SSML document to be synthesized. The SSML document must be valid
        /// and well-formed. Otherwise the RPC will fail and return
        /// \[google.rpc.Code.INVALID_ARGUMENT][\]. For more information, see
        /// \[SSML\](<https://cloud.google.com/text-to-speech/docs/ssml>).
        #[prost(string, tag = "2")]
        Ssml(::prost::alloc::string::String),
    }
}
/// Description of which voice to use for a synthesis request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceSelectionParams {
    /// Required. The language (and potentially also the region) of the voice expressed as a
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag, e.g.
    /// "en-US". This should not include a script tag (e.g. use
    /// "cmn-cn" rather than "cmn-Hant-cn"), because the script will be inferred
    /// from the input provided in the SynthesisInput.  The TTS service
    /// will use this parameter to help choose an appropriate voice.  Note that
    /// the TTS service may choose a voice with a slightly different language code
    /// than the one selected; it may substitute a different region
    /// (e.g. using en-US rather than en-CA if there isn't a Canadian voice
    /// available), or even a different language, e.g. using "nb" (Norwegian
    /// Bokmal) instead of "no" (Norwegian)".
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
    /// The name of the voice. If not set, the service will choose a
    /// voice based on the other parameters such as language_code and gender.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The preferred gender of the voice. If not set, the service will
    /// choose a voice based on the other parameters such as language_code and
    /// name. Note that this is only a preference, not requirement; if a
    /// voice of the appropriate gender is not available, the synthesizer should
    /// substitute a voice with a different gender rather than failing the request.
    #[prost(enumeration = "SsmlVoiceGender", tag = "3")]
    pub ssml_gender: i32,
}
/// Description of audio data to be synthesized.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioConfig {
    /// Required. The format of the audio byte stream.
    #[prost(enumeration = "AudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Optional. Input only. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is
    /// the normal native speed supported by the specific voice. 2.0 is twice as
    /// fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0
    /// speed. Any other values < 0.25 or > 4.0 will return an error.
    #[prost(double, tag = "2")]
    pub speaking_rate: f64,
    /// Optional. Input only. Speaking pitch, in the range [-20.0, 20.0]. 20 means
    /// increase 20 semitones from the original pitch. -20 means decrease 20
    /// semitones from the original pitch.
    #[prost(double, tag = "3")]
    pub pitch: f64,
    /// Optional. Input only. Volume gain (in dB) of the normal native volume
    /// supported by the specific voice, in the range [-96.0, 16.0]. If unset, or
    /// set to a value of 0.0 (dB), will play at normal native signal amplitude. A
    /// value of -6.0 (dB) will play at approximately half the amplitude of the
    /// normal native signal amplitude. A value of +6.0 (dB) will play at
    /// approximately twice the amplitude of the normal native signal amplitude.
    /// Strongly recommend not to exceed +10 (dB) as there's usually no effective
    /// increase in loudness for any value greater than that.
    #[prost(double, tag = "4")]
    pub volume_gain_db: f64,
    /// Optional. The synthesis sample rate (in hertz) for this audio. When this is
    /// specified in SynthesizeSpeechRequest, if this is different from the voice's
    /// natural sample rate, then the synthesizer will honor this request by
    /// converting to the desired sample rate (which might result in worse audio
    /// quality), unless the specified sample rate is not supported for the
    /// encoding chosen, in which case it will fail the request and return
    /// \[google.rpc.Code.INVALID_ARGUMENT][\].
    #[prost(int32, tag = "5")]
    pub sample_rate_hertz: i32,
    /// Optional. Input only. An identifier which selects 'audio effects' profiles
    /// that are applied on (post synthesized) text to speech. Effects are applied
    /// on top of each other in the order they are given. See
    /// [audio
    /// profiles](<https://cloud.google.com/text-to-speech/docs/audio-profiles>) for
    /// current supported profile ids.
    #[prost(string, repeated, tag = "6")]
    pub effects_profile_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The message returned to the client by the `SynthesizeSpeech` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechResponse {
    /// The audio data bytes encoded as specified in the request, including the
    /// header for encodings that are wrapped in containers (e.g. MP3, OGG_OPUS).
    /// For LINEAR16 audio, we include the WAV header. Note: as
    /// with all bytes fields, protobuffers use a pure binary representation,
    /// whereas JSON representations use base64.
    #[prost(bytes = "vec", tag = "1")]
    pub audio_content: ::prost::alloc::vec::Vec<u8>,
}
/// Gender of the voice as described in
/// [SSML voice element](<https://www.w3.org/TR/speech-synthesis11/#edef_voice>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SsmlVoiceGender {
    /// An unspecified gender.
    /// In VoiceSelectionParams, this means that the client doesn't care which
    /// gender the selected voice will have. In the Voice field of
    /// ListVoicesResponse, this may mean that the voice doesn't fit any of the
    /// other categories in this enum, or that the gender of the voice isn't known.
    Unspecified = 0,
    /// A male voice.
    Male = 1,
    /// A female voice.
    Female = 2,
    /// A gender-neutral voice. This voice is not yet supported.
    Neutral = 3,
}
impl SsmlVoiceGender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SsmlVoiceGender::Unspecified => "SSML_VOICE_GENDER_UNSPECIFIED",
            SsmlVoiceGender::Male => "MALE",
            SsmlVoiceGender::Female => "FEMALE",
            SsmlVoiceGender::Neutral => "NEUTRAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SSML_VOICE_GENDER_UNSPECIFIED" => Some(Self::Unspecified),
            "MALE" => Some(Self::Male),
            "FEMALE" => Some(Self::Female),
            "NEUTRAL" => Some(Self::Neutral),
            _ => None,
        }
    }
}
/// Configuration to set up audio encoder. The encoding determines the output
/// audio format that we'd like.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioEncoding {
    /// Not specified. Will return result \[google.rpc.Code.INVALID_ARGUMENT][\].
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    /// Audio content returned as LINEAR16 also contains a WAV header.
    Linear16 = 1,
    /// MP3 audio at 32kbps.
    Mp3 = 2,
    /// Opus encoded audio wrapped in an ogg container. The result will be a
    /// file which can be played natively on Android, and in browsers (at least
    /// Chrome and Firefox). The quality of the encoding is considerably higher
    /// than MP3 while using approximately the same bitrate.
    OggOpus = 3,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    /// Audio content returned as MULAW also contains a WAV header.
    Mulaw = 5,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/A-law.
    /// Audio content returned as ALAW also contains a WAV header.
    Alaw = 6,
}
impl AudioEncoding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AudioEncoding::Unspecified => "AUDIO_ENCODING_UNSPECIFIED",
            AudioEncoding::Linear16 => "LINEAR16",
            AudioEncoding::Mp3 => "MP3",
            AudioEncoding::OggOpus => "OGG_OPUS",
            AudioEncoding::Mulaw => "MULAW",
            AudioEncoding::Alaw => "ALAW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUDIO_ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
            "LINEAR16" => Some(Self::Linear16),
            "MP3" => Some(Self::Mp3),
            "OGG_OPUS" => Some(Self::OggOpus),
            "MULAW" => Some(Self::Mulaw),
            "ALAW" => Some(Self::Alaw),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod text_to_speech_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that implements Google Cloud Text-to-Speech API.
    #[derive(Debug, Clone)]
    pub struct TextToSpeechClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TextToSpeechClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TextToSpeechClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TextToSpeechClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Returns a list of Voice supported for synthesis.
        pub async fn list_voices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVoicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVoicesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.texttospeech.v1.TextToSpeech/ListVoices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.texttospeech.v1.TextToSpeech",
                        "ListVoices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Synthesizes speech synchronously: receive results after all text input
        /// has been processed.
        pub async fn synthesize_speech(
            &mut self,
            request: impl tonic::IntoRequest<super::SynthesizeSpeechRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SynthesizeSpeechResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.texttospeech.v1.TextToSpeech/SynthesizeSpeech",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.texttospeech.v1.TextToSpeech",
                        "SynthesizeSpeech",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
