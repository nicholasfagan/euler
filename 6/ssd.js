var readline = require('readline');
var rl = readline.createInterface({
	input:process.stdin,
	output:process.stdout
});


const sum = (a,c)=>a+c;
const sq = x=>x*x;


rl.on('line',input=>{
	console.log(parseInt(input)<=0||isNaN(parseInt(input))?0:
		sq(Array.from({length:parseInt(input)},(a,i)=>i+1)
		.reduce(sum))
		-
		Array.from({length:parseInt(input)},(a,i)=>i+1)
			.map(sq).reduce(sum)
	);
});
