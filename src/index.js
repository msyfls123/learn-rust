import loadAdd from './lib.rs'


loadAdd().then(result => {
  const { add, subtract, multiply } = result.instance.exports;
  //console.log(add, subtract, multiply, add(1,2))
  console.log('1 + 1 = ', add(1, 1))
  console.log('1 - 1 = ', subtract(1, 1))
  console.log('2 * 2 = ', multiply(2, 2))
});
