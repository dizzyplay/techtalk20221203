const leven = require('.').leven;
const { wordList } = require('./wordlist.js');
const readline = require('readline');

const command = wordList.split('\n').filter( v => v !== '');
console.info( '저장된 단어개수: ' + command.length);

const rl = readline.createInterface({
	input: process.stdin,
	output: process.stdout,
});

rl.question('Input your command -> ', answer => {
	console.time('time');
	const partial = command.filter(v => {
		const result = leven(v,answer);
		return result  < 2 && result >= 0;
	});
	console.timeEnd('time');

	rl.close();
	console.info(`did you mean this ? -> ${partial.reduce((acc,v) => acc += v + ' ', '')}`);
});
