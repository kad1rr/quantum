import chalk from 'chalk'
import os from 'os'
import amdLogo from './assets/amd.txt'
import intelLogo from './assets/intel.txt'
const pidusage = (await import('pidusage')).default

const CPU = os.cpus()

const getCPUModel = () => {
  if (CPU[0].model.toLowerCase().startsWith('amd')) {
    return 'amd'
  } else if (CPU[0].model.toLowerCase().startsWith('intel')) {
    return 'intel'
  }
}

const header = () => {
  console.clear()
  if (getCPUModel() === 'amd') {
    console.log(chalk.red(amdLogo))
  } else if (getCPUModel() === 'intel') {
    console.log(chalk.blue(intelLogo))
  } else {
    console.log(chalk.green('Quantum'))
  }
}

const getCPUUsage = () => {
  let usage = 0
  pidusage(process.pid, (err, stat) => {
    usage = stat.cpu
  })
  return usage
}

const cpuUsageData =
  '~' +
  getCPUUsage() +
  '%' +
  `     ${
    getCPUUsage() < 20
      ? chalk.green('(Perfect)')
      : getCPUUsage() < 45
        ? chalk.blue('(Good)')
        : getCPUUsage() < 75
          ? chalk.yellow('(Medium)')
          : chalk.red('(Bad)')
  }`

const data = [
  ['Cpu Model', CPU[0].model],
  ['Cpu Usage', cpuUsageData],
  ['Cpu Times', CPU[0].times.user],
  ['Memory Usage', process.memoryUsage().rss / 1024 + ' GB'],
]

header()
data.forEach(io => {
  if (getCPUModel() === 'amd') {
    console.log(chalk.red(io[0] + ':'), chalk.cyan(io[1]))
  } else {
    console.log(chalk.blue(io[0] + ':'), chalk.cyan(io[1]))
  }
})

while (true) {}
