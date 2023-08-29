import 'package:flutter/material.dart';

/// Offsets a given color linearly by a given offset
/// Does not change opacity
/// Offset is added; negative values will decrease color
Color offsetColor(Color color, int offset) {
  return Color.fromRGBO(
    constrainColor(color.red + offset),
    constrainColor(color.green + offset),
    constrainColor(color.blue + offset),
    color.opacity,
  );
}

/// prevent color value from going out of bounds
int constrainColor(int value) {
  if (value < 0) {
    return 0;
  } else if (value > 255) {
    return 255;
  } else {
    return value;
  }
}
