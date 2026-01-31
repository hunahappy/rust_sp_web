function init_page() {
    $$("combo_dan").setValue("m10");
    $$("combo_juya").setValue("all");

    const today = new Date();
    const sevenDaysAgo = new Date();
    sevenDaysAgo.setDate(today.getDate() - 7);

    $$("startDate").setValue(sevenDaysAgo);
    $$("endDate").setValue(today);

    on_search();
}

function on_search() {
    const dan = $$("combo_dan").getValue();
    const juya = $$("combo_juya").getValue();

    const startDate = $$("startDate").getValue();
    const endDate = $$("endDate").getValue();

    const send_json = { "단계": dan, "주야": juya, "시작일": startDate, "종료일": endDate };
    webix.ajax().headers({ "Content-type": "application/json" }).post("/report_th_1/post_chart_1", send_json, function (text) {
        var json_obj = JSON.parse(text);

        $$("chart1").clearAll();
        $$("chart2").clearAll();
        $$("chart3").clearAll();

        $$("chart1").parse(json_obj);
        $$("chart2").parse(json_obj);
        $$("chart3").parse(json_obj);
    });
}

