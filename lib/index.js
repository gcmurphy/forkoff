const plugin = require('../native');
const { exec } = require('child_process');

console.log(plugin.forkoff())

console.log("About to call external process");
exec("cat /etc/passwd", (err, stdout, stderr) => {
	console.log("command seems to have run...");
	return;
})
console.log("If this exited normally then we're back to the drawing board...")
