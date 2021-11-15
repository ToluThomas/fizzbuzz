const n = + process.argv[2];
const f = 'Fizz';
const b = 'Buzz';
const fb = 'FizzBuzz';

for (var i = 1; i < n + 1; i++) {
  if (i % 3 == 0 && i % 5 == 0) {
    process.stdout.write(fb);
  } else if (i % 3 == 0) {
    process.stdout.write(f);
  } else if (i % 5 == 0) {
    process.stdout.write(b);
  } else {
    process.stdout.write(i + '');
  }
}
