use crate::{methods, util, Result};
use dbus::{
    arg::{Append, Arg, Get, RefArg},
    nonblock::{Proxy, SyncConnection},
};
use std::{fmt::Display, time::Duration};

/// A struct used to control an MPRIS player.
#[derive(Clone)]
pub struct Player<'a> {
    pub name: String,
    pub conn: &'a SyncConnection,
}

impl<'a> Player<'a> {
    /// Tries to create a new `Player` instance from a given name.
    ///
    /// # Errors
    /// Returns an `Err` if the provided player is invalid.
    pub async fn try_new<T>(name: T, conn: &'a SyncConnection) -> Result<Player<'a>>
    where
        T: AsRef<str> + Display,
    {
        if !util::validate(name.as_ref(), conn).await? {
            return Err(Box::from("The provided player was invalid."));
        }

        let player = Player {
            name: name.to_string(),
            conn,
        };
        Ok(player)
    }

    #[doc(hidden)]
    pub fn get_proxy(&mut self) -> Result<Proxy<&'a SyncConnection>> {
        let proxy = Proxy::new(
            format!("org.mpris.MediaPlayer2.{}", self.name),
            "/org/mpris/MediaPlayer2",
            Duration::from_millis(5000),
            self.conn,
        );

        Ok(proxy)
    }

    /// Skips to the next track
    pub async fn next(&mut self) {
        methods::next(self).await.unwrap();
    }

    /// Skips to the previous track
    pub async fn previous(&mut self) {
        methods::previous(self).await.unwrap();
    }

    /// Pauses the current track
    pub async fn pause(&mut self) {
        methods::pause(self).await.unwrap();
    }

    /// Starts or resumes the current track
    pub async fn play(&mut self) {
        methods::play(self).await.unwrap();
    }

    /// Resumes/starts or pauses the current track
    pub async fn play_pause(&mut self) {
        methods::play_pause(self).await.unwrap();
    }

    /// Stops playback
    pub async fn stop(&mut self) {
        methods::stop(self).await.unwrap();
    }

    /// Retrieves a metadata property from the given player.
    ///
    /// # Errors
    /// May return an `Err` variant if the provided property was invalid.
    pub async fn get_metadata_property(&mut self, property: &str) -> Result<Box<dyn RefArg>> {
        Ok(methods::get_metadata_property(self, property).await?)
    }

    /// Retrieves the value of an MPRIS property.
    /// Available properties can be found [here].
    ///
    /// [here]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
    ///
    /// # Errors
    /// May return an `Err` variant if:
    /// * An invalid type was provided for the property
    /// * An invalid property was provided
    pub async fn get_property<T>(&mut self, property: &str) -> Result<T>
    where
        T: for<'c> Get<'c> + 'static,
    {
        Ok(methods::get_property(self, property).await?)
    }

    /// Sets the value of a writable MPRIS property.
    /// Available properties can be found [here].
    ///
    /// [here]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
    ///
    /// # Errors
    /// May return an `Err` variant if:
    /// * An invalid type was provided for the property
    /// * An invalid property was provided
    pub async fn set_property<T>(&mut self, property: &str, value: T) -> Result<()>
    where
        T: Arg + Append,
    {
        Ok(methods::set_property(self, property, value).await?)
    }

    /// Seeks the position of the active track.
    pub async fn seek(&mut self, offset: Duration) -> Result<()> {
        Ok(methods::seek(self, offset).await?)
    }

    /// Same as `seek`, but in reverse.
    pub async fn seek_reverse(&mut self, offset: Duration) -> Result<()> {
        Ok(methods::seek_reverse(self, offset).await?)
    }

    /// Sets the position of the current track, by microseconds.
    pub async fn set_position(&mut self, position: i64) -> Result<()> {
        Ok(methods::set_position(self, position).await?)
    }

    /// Opens a track by its URI.
    ///
    /// # Errors
    /// May return an `Err` variant if the provided URI is invalid.
    pub async fn open_uri(&mut self, uri: &str) -> Result<()> {
        Ok(methods::open_uri(self, uri).await?)
    }
}
