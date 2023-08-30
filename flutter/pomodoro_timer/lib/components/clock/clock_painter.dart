import 'dart:math';

import 'package:flutter/material.dart';

class ClockPainter extends CustomPainter {
  const ClockPainter({
    required this.brightness,
    required this.baseTime,
    required this.numMillis,
  });

  final Brightness brightness;
  final int baseTime;
  final int numMillis;

  @override
  void paint(Canvas canvas, Size size) {
    // Draw clock face
    final centerX = size.width / 2;
    final centerY = size.height / 2;
    final centerOffset = Offset(centerX, centerY);
    final radius = size.width / 2;
    final numSeconds = numMillis / 1000;

    // Draw second hand
    final secondHandPaint = Paint()
      ..color = const Color(0xffff0000)
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = 2;
    final secondRadians =
        -((numSeconds % 60) * 6 + 90 - ((baseTime / 1000) % 60) * 6) *
            (pi / 180);
    final secondHandX = centerX + (radius - 20) * cos(secondRadians);
    final secondHandY = centerY + (radius - 20) * sin(secondRadians);
    final secondHandOffset = Offset(secondHandX, secondHandY);
    canvas.drawLine(centerOffset, secondHandOffset, secondHandPaint);

    // Draw total hand
    final hourHandPaint = Paint()
      ..color = (brightness == Brightness.light) ? Colors.black : Colors.white
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = 6;
    final hourRadians = -((numMillis / baseTime) * 360 + 90) * (pi / 180);
    final hourHandX = centerX + (radius - 30) * cos(hourRadians);
    final hourHandY = centerY + (radius - 30) * sin(hourRadians);
    final hourHandOffset = Offset(hourHandX, hourHandY);
    canvas.drawLine(centerOffset, hourHandOffset, hourHandPaint);
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}
