enum numCyclesOptions {
  one('1', 1),
  two('2', 2),
  three('3', 3),
  four('4', 4),
  five('5', 5),
  six('6', 6);

  const numCyclesOptions(this.label, this.numCycles);
  final String label;
  final int numCycles;
}

enum workTimerOptions {
  fifteen('15 minutes', 900000),
  twenty('20 minutes', 1200000),
  twentyfive('25 minutes', 1500000),
  thirty('30 minutes', 1800000),
  fourtyfive('45 minutes', 2700000),
  fifty('50 minutes', 3000000),
  sixty('60 minutes', 3600000);

  const workTimerOptions(this.label, this.workTime);
  final String label;
  final int workTime;
}

enum shortBreakTimerOptions {
  five('5 minutes', 300000),
  ten('10 minutes', 600000);

  const shortBreakTimerOptions(this.label, this.shortBreakTime);
  final String label;
  final int shortBreakTime;
}

enum longBreakTimerOptions {
  fifteen('15 minutes', 900000),
  twenty('20 minutes', 1200000),
  thirty('30 minutes', 1800000);

  const longBreakTimerOptions(this.label, this.longBreakTime);
  final String label;
  final int longBreakTime;
}
