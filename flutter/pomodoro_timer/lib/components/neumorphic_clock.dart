import 'package:flutter/material.dart';
import 'package:pomodoro_timer/components/clock_painter.dart';
import 'package:pomodoro_timer/utils/defaults.dart';
import 'package:shared_preferences/shared_preferences.dart';

class NeumorphicClock extends StatefulWidget {
  const NeumorphicClock({
    super.key,
    required this.currentDot,
    required this.numMillis,
    required this.color,
  });

  final int currentDot;
  final int numMillis;
  final Color color;

  State<NeumorphicClock> createState() => _NeumorphicClockState();
}

class _NeumorphicClockState extends State<NeumorphicClock> {
  int _workTime = defaultWorkTime;
  int _shortBreakTime = defaultShortBreakTime;
  int _longBreakTime = defaultLongBreakTime;

  int _numCycles = 2;

  @override
  void initState() {
    super.initState();
    _loadTimers();
  }

  Future<void> _loadTimers() async {
    // obtain shared preferences
    final prefs = await SharedPreferences.getInstance();

    setState(() {
      _numCycles = prefs.getInt('numCycles') ?? 2;

      _workTime = prefs.getInt('workTime') ?? defaultWorkTime;
      _shortBreakTime = prefs.getInt('shortBreakTime') ?? defaultShortBreakTime;
      _longBreakTime = prefs.getInt('longBreakTIme') ?? defaultLongBreakTime;
    });
  }

  @override
  Widget build(BuildContext context) {
    int baseTime = 0;

    if (widget.currentDot == _numCycles * 2 + 1) {
      baseTime = _longBreakTime;
    } else if (widget.currentDot % 2 == 0) {
      baseTime = _workTime;
    } else {
      baseTime = _shortBreakTime;
    }

    return AnimatedContainer(
      duration: const Duration(seconds: 1),
      height: 250,
      width: 250,
      padding: const EdgeInsets.all(25),
      decoration: BoxDecoration(
        color: widget.color,
        shape: BoxShape.circle,
        boxShadow: [
          const BoxShadow(
            offset: Offset(6.25, 16),
            color: Colors.black38,
            blurRadius: 25,
            spreadRadius: 0.5,
          ),
          BoxShadow(
            offset: const Offset(-6.25, -12.5),
            color: (Theme.of(context).brightness == Brightness.light)
                ? Colors.white38
                : Colors.white12,
            blurRadius: 37.5,
            spreadRadius: 0.1,
          ),
        ],
      ),
      child: CustomPaint(
        painter: ClockPainter(
          brightness: Theme.of(context).brightness,
          baseTime: baseTime,
          numMillis: widget.numMillis,
        ),
      ),
    );
  }
}
