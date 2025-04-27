(async () => {
  console.log('example');

  await chrome.runtime.sendMessage<object, string>({
    action: 'message from popup',
  });
})();
