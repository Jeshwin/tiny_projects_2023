import 'package:flutter/material.dart' hide BoxDecoration, BoxShadow;
import 'package:flutter_inset_box_shadow/flutter_inset_box_shadow.dart';
import 'package:pomodoro_timer/components/clock_painter.dart';
import 'package:pomodoro_timer/utils/constants.dart';

class NeumorphicClock extends StatelessWidget {
  const NeumorphicClock({
    super.key,
    required this.numCycles,
    required this.currentDot,
    required this.numMillis,
    required this.color,
  });

  final int numCycles;
  final int currentDot;
  final int numMillis;
  final Color color;

  @override
  Widget build(BuildContext context) {
    int baseTime = 0;

    if (currentDot == numCycles * 2 + 1) {
      baseTime = LONG_BREAK_TIME;
    } else if (currentDot % 2 == 0) {
      baseTime = WORK_TIME;
    } else {
      baseTime = SHORT_BREAK_TIME;
    }

    return Container(
      height: 250,
      width: 250,
      padding: const EdgeInsets.all(25),
      decoration: BoxDecoration(
        color: color,
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
          numMillis: numMillis,
        ),
      ),
    );
  }
}
