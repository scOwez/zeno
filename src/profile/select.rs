//! # About
//! 
//! Basic profile management for the user, gives a popup for frontend CRD (create,
//! read, delete) operations on a profile.
//! 
//! If you'd like profile *editing* options, please see [crate::profile::options].

use crate::editor::screen::editor_screen;
use crate::StartMeta;
use cursive::views::{Button, Dialog, EditView, LinearLayout, SelectView};
use cursive::{traits::*, Cursive};

/// Profile selector for multi-user/multi-purpose editing (allowing for more
/// flexible options).
pub fn profile_select(s: &mut Cursive, meta: StartMeta) {
    let profile_list = SelectView::<String>::new()
        .on_submit(move |s, selected_item| {
            editor_screen(s, selected_item, &meta);
        })
        .with_id("p_list")
        .fixed_size((32, 8));
    let admin_buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_profile))
        .child(Button::new("Remove", remove_conf));

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(profile_list)
                .child(admin_buttons),
        )
        .title("Profile selector"),
    )
}

/// Confirmation popup to make sure user really wants to delete their profile.
///
/// This should ideally be embedded *inside* of [remove_profile] in the future.
/// See <https://gitlab.com/zeno-src/zeno/issues/9> for more infomation.
fn remove_conf(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Are you sure you want to delete the selected profile?")
            .button("Yes", remove_profile)
            .button("No", |s| {
                s.pop_layer();
            }),
    )
}

/// Allows a user to delete/remove a profile.
fn remove_profile(s: &mut Cursive) {
    s.pop_layer();

    let mut got_select = s.find_id::<SelectView<String>>("p_list").unwrap();

    match got_select.selected_id() {
        None => s.add_layer(Dialog::info("No profiles to remove!")),
        Some(profile) => {
            got_select.remove_item(profile);
        }
    }
}

/// Allows a user to create a new profile.
fn add_profile(s: &mut Cursive) {
    /// Adds a name to the profile list ([SelectView])
    fn add_to_list(s: &mut Cursive, p_name: &str) {
        if p_name == "" {
            s.add_layer(Dialog::info("Cannot add a new profile with no name!"));
        } else {
            s.call_on_id("p_list", |view: &mut SelectView<String>| {
                view.add_item_str(p_name);
            });
            s.pop_layer();
        }
    }

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(add_to_list)
                .with_id("p_name")
                .fixed_width(32),
        )
        .title("Add new profile")
        .button("Ok", |s| {
            let p_name = s
                .call_on_id("p_name", |view: &mut EditView| view.get_content())
                .unwrap(); // Get content from EditView
            add_to_list(s, &p_name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    )
}
