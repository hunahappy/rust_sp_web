function init_page() {
    on_search();
}

function jsonToTree(obj, parent = "root") {
    if (typeof obj !== "object" || obj === null) {
        return { id: webix.uid(), value: String(obj) };
    }

    const node = {
        id: webix.uid(),
        value: parent,
        data: []
    };

    for (let k in obj) {
        if (typeof obj[k] === "object") {
            node.data.push(jsonToTree(obj[k], k));
        } else {
            node.data.push({
                id: webix.uid(),
                value: k + ": " + obj[k]
            });
        }
    }
    return node;
}

function on_search() {
    const send_json = {};
    webix.ajax().headers({ "Content-type": "application/json" }).post("/system_info/get_system_info", send_json, function (text) {
        var json_obj = JSON.parse(text);

        const treeData = jsonToTree(json_obj, "status");

        $$("tr1").parse([treeData]);
        $$("tr1").openAll();
    });
}

