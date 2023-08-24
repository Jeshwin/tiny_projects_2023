import 'package:flutter/material.dart';
import 'package:pomodoro_timer/components/clock_painter.dart';

class NeumorphicClock extends StatefulWidget {
  const NeumorphicClock({
    super.key,
    required this.numCycles,
    required this.currentDot,
    required this.numMillis,
    required this.workTime,
    required this.shortBreakTime,
    required this.longBreakTime,
    required this.color,
  });

  final int numCycles;
  final int currentDot;
  final int numMillis;
  final int workTime;
  final int shortBreakTime;
  final int longBreakTime;
  final Color color;

  @override
  State<NeumorphicClock> createState() => _NeumorphicClockState();
}

class _NeumorphicClockState extends State<NeumorphicClock> {
  @override
  Widget build(BuildContext context) {
    int baseTime = 0;

    if (widget.currentDot == widget.numCycles * 2 + 1) {
      baseTime = widget.longBreakTime;
    } else if (widget.currentDot % 2 == 0) {
      baseTime = widget.workTime;
    } else {
      baseTime = widget.shortBreakTime;
    }

    return AnimatedContainer(
      duration: const Duration(milliseconds: 700),
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
