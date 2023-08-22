import 'package:flutter/material.dart';
import '../styles/catppuccin.dart';

class PomodoroDots extends StatelessWidget {
  const PomodoroDots({
    super.key,
    required this.numCycles,
    this.currentDot = 0,
  });

  final int numCycles;
  final int currentDot;

  @override
  Widget build(BuildContext context) {
    List<Widget> childrenDots = [];
    int completed = currentDot;
    IconData iconToAdd = Icons.circle_outlined;

    // Add work and short break
    for (var i = 0; i < numCycles; i++) {
      if (completed > 0) {
        iconToAdd = Icons.circle;
      } else {
        iconToAdd = Icons.circle_outlined;
      }
      childrenDots.add(Icon(
        iconToAdd,
        color: Catppuccin().green,
      ));
      completed--;
      if (completed > 0) {
        iconToAdd = Icons.circle;
      } else {
        iconToAdd = Icons.circle_outlined;
      }
      childrenDots.add(Icon(
        iconToAdd,
        color: Catppuccin().teal,
      ));
      completed--;
    }

    // Add long break
    if (completed > 0) {
      iconToAdd = Icons.circle;
    } else {
      iconToAdd = Icons.circle_outlined;
    }
    childrenDots.add(Icon(
      iconToAdd,
      color: Catppuccin().mauve,
    ));

    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: childrenDots,
    );
    ;
  }
}
