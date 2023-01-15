use crate::app_util::App;

mod app_util;

#[test]
fn it_adds_and_lists_items() {
    let app = App::from_temp_file();

    let add_item_assertion = app.add("item 1");
    let list_items_assertion = app.ls();

    add_item_assertion.stdout("OK\n");
    list_items_assertion.stdout("item 1\n");
}
