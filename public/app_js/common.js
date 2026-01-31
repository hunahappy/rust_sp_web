var top_menu = {
    view: "toolbar",
    height: 50,
    css: "webix_dark",
    elements: [
        {
            view: "menu",
            layout: "x",
            data: [
                {
                    id: "report",
                    value: "리포트",
                    submenu: [
                        { id: "th1", value: "온도 습도 조도 리포트" },
                        { id: "th2", value: "누적 온도 조도" },
                        { id: "system_info", value: "시스템 정보" },
                    ]
                },
                {
                    id: "gita",
                    value: "기타",
                    submenu: [
                        { id: "error", value: "에러" },
                    ]
                }
            ],
            on: {                
                onMenuItemClick: function (id) {
                    if (id == "th1") { location.href = '/report_th_1'; }
                    else if (id == "th2") { location.href = '/report_th_2'; }
                    else if (id == "system_info") { location.href = '/system_info'; }
                    else if (id == "error") { location.href = '/error'; }
                }
            }
        }
    ]
};

function comma1000(num) {
    num = String(num);
    return num.replace(/(\d)(?=(?:\d{3})+(?!\d))/g, "$1,");
}