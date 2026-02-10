use super::{
  super::app::{App, DialogContext, PlaylistFolderItem, TrackTableContext},
  common_key_events,
};
use crate::app::{ActiveBlock, RouteId};
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
    k if common_key_events::down_event(k) => {
      let display_items = app.get_playlist_display_items();
      if !display_items.is_empty() {
        if let Some(selected_playlist_index) = app.selected_playlist_index {
          let next_index =
            common_key_events::on_down_press_handler(&display_items, Some(selected_playlist_index));
          app.selected_playlist_index = Some(next_index);
        }
      }
    }
    k if common_key_events::up_event(k) => {
      let display_items = app.get_playlist_display_items();
      if !display_items.is_empty() {
        let next_index =
          common_key_events::on_up_press_handler(&display_items, app.selected_playlist_index);
        app.selected_playlist_index = Some(next_index);
      }
    }
    k if common_key_events::high_event(k) => {
      let display_items = app.get_playlist_display_items();
      if !display_items.is_empty() {
        let next_index = common_key_events::on_high_press_handler();
        app.selected_playlist_index = Some(next_index);
      }
    }
    k if common_key_events::middle_event(k) => {
      let display_items = app.get_playlist_display_items();
      if !display_items.is_empty() {
        let next_index = common_key_events::on_middle_press_handler(&display_items);
        app.selected_playlist_index = Some(next_index);
      }
    }
    k if common_key_events::low_event(k) => {
      let display_items = app.get_playlist_display_items();
      if !display_items.is_empty() {
        let next_index = common_key_events::on_low_press_handler(&display_items);
        app.selected_playlist_index = Some(next_index);
      }
    }
    Key::Enter => {
      let display_items = app.get_playlist_display_items();
      if let Some(selected_idx) = app.selected_playlist_index {
        if let Some(item) = display_items.get(selected_idx) {
          match item {
            PlaylistFolderItem::Folder(folder) => {
              // Navigate into/out of folder
              app.current_playlist_folder_id = folder.target_id;
              app.selected_playlist_index = Some(0);
            }
            PlaylistFolderItem::Playlist { index, .. } => {
              // Open the playlist tracks
              if let Some(playlist) = app.all_playlists.get(*index) {
                app.active_playlist_index = Some(*index);
                app.track_table.context = Some(TrackTableContext::MyPlaylists);
                app.playlist_offset = 0;
                let playlist_id = playlist.id.clone().into_static();
                app.dispatch(IoEvent::GetPlaylistItems(
                  playlist_id.clone(),
                  app.playlist_offset,
                ));
                // Pre-fetch more pages in background for seamless playback
                app.dispatch(IoEvent::PreFetchAllPlaylistTracks(playlist_id));
              }
            }
          }
        }
      }
    }
    Key::Char('D') => {
      let display_items = app.get_playlist_display_items();
      if let Some(selected_idx) = app.selected_playlist_index {
        if let Some(PlaylistFolderItem::Playlist { index, .. }) = display_items.get(selected_idx) {
          if let Some(playlist) = app.all_playlists.get(*index) {
            let selected_playlist = &playlist.name;
            app.dialog = Some(selected_playlist.clone());
            app.confirm = false;

            app.push_navigation_stack(
              RouteId::Dialog,
              ActiveBlock::Dialog(DialogContext::PlaylistWindow),
            );
          }
        }
      }
    }
    _ => {}
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {}
}
