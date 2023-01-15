use crate::app_util::App;

mod app_util;

#[test]
fn it_should_add_and_remove_items() {
    let app = App::from_temp_file();

    app.add("item 1");
    let ls_1_assertion = app.ls();
    app.rm("item 1");
    let ls_2_assertion = app.ls();

    ls_1_assertion.stdout("item 1\n");
    ls_2_assertion.stdout("The list is empty\n");
}
