import 'package:flutter/material.dart';
import 'package:pomodoro_timer/components/clock/clock_painter.dart';
import 'package:pomodoro_timer/utils/helper_functions.dart';
import 'package:pomodoro_timer/utils/settings_model.dart';
import 'package:provider/provider.dart';

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

  @override
  State<NeumorphicClock> createState() => _NeumorphicClockState();
}

class _NeumorphicClockState extends State<NeumorphicClock> {
  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 700),
      height: 250,
      width: 250,
      padding: const EdgeInsets.all(25),
      decoration: BoxDecoration(
        color: widget.color,
        shape: BoxShape.circle,
        boxShadow: [
          BoxShadow(
            offset: const Offset(6.25, 16),
            color: offsetColor(widget.color, -21),
            blurRadius: 25,
          ),
          BoxShadow(
            offset: const Offset(-6.25, -12.5),
            color: offsetColor(widget.color, 21),
            blurRadius: 37.5,
          ),
        ],
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            offsetColor(widget.color, -7),
            offsetColor(widget.color, 7),
          ],
        ),
      ),
      child: Consumer<SettingsModel>(
        builder: (context, settings, child) {
          int baseTime = 0;

          if (widget.currentDot == settings.numCycles * 2 + 1) {
            baseTime = settings.longBreakTime;
          } else if (widget.currentDot % 2 == 0) {
            baseTime = settings.workTime;
          } else {
            baseTime = settings.shortBreakTime;
          }

          return CustomPaint(
            painter: ClockPainter(
              brightness: Theme.of(context).brightness,
              baseTime: baseTime,
              numMillis: widget.numMillis,
            ),
          );
        },
      ),
    );
  }
}
