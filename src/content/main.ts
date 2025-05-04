(async () => {
  await chrome.runtime.sendMessage<object, string>({
    action: 'message from content script',
  });
})();

chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
  console.log('Message received in content script:', request, sender);
  sendResponse({ reply: 'response from content script' });
});
