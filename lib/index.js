const plugin = require('../native');
const { exec } = require('child_process');

plugin.forkoff();
console.log("calling..");
exec("cat /etc/passwd", (err, stdout, stderr) => {
	console.log(err)
	console.log(stdout)
	console.log(stderr)
	console.log("command ran...");
	return {};
})
console.log("didn't work")
