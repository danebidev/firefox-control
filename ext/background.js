let port = browser.runtime.connectNative("firefox_control");

port.onMessage.addListener((message) => {
    switch (message.command) {
        case "list":
            browser.tabs.query({}).then((tabs) => {
                let tabTitles = tabs.map((tab) => {
                    return tab.index + ". " + tab.title;
                });
                port.postMessage({
                    titles: tabTitles,
                });
            });
            break;
        case "open":
            let url = message.url;
            browser.tabs.create({
                url: url,
            });
            break;
        case "close":
            let tabIndex = message.index;
            browser.tabs.query({}).then((tabs) => {
                let tabId = tabs[tabIndex].id;
                browser.tabs.remove(tabId);
            });
            break;
        case "select":
            browser.tabs.query({}).then((tabs) => {
                let tabId = tabs[message.index].id;
                browser.tabs.update(tabId, {
                    active: true,
                });
            });
            break;
    }
});

port.onDisconnect.addListener(() => {
    console.log("Disconnected from native app");
    console.error(port.error);
});
