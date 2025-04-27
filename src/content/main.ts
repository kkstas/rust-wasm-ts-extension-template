(async () => {
  await chrome.runtime.sendMessage<object, string>({
    action: 'message from content script',
  });
})();
