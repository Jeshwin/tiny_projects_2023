import 'package:flutter/material.dart';

class PomodoroDots extends StatelessWidget {
  const PomodoroDots({
    super.key,
    required this.numCycles,
    required this.currentDot,
    required this.isRunning,
  });

  final int numCycles;
  final int currentDot;
  final bool isRunning;

  @override
  Widget build(BuildContext context) {
    List<Widget> childrenDots = [];
    int completed = currentDot;
    IconData iconToAdd = Icons.circle_outlined;

    // Add work and short break
    for (var i = 0; i <= numCycles * 2; i++) {
      if (completed > 0) {
        iconToAdd = Icons.circle;
      } else if (completed == 0) {
        if (isRunning) {
          iconToAdd = Icons.square;
        } else {
          iconToAdd = Icons.circle_outlined;
        }
      } else {
        iconToAdd = Icons.circle_outlined;
      }
      childrenDots.add(Icon(
        iconToAdd,
      ));
      completed--;
    }

    // Add long break
    if (completed > 0) {
      iconToAdd = Icons.circle;
    } else if (completed == 0) {
      if (isRunning) {
        iconToAdd = Icons.square;
      } else {
        iconToAdd = Icons.circle_outlined;
      }
    } else {
      iconToAdd = Icons.circle_outlined;
    }
    childrenDots.add(Icon(
      iconToAdd,
    ));

    String currentText;

    if (currentDot == numCycles * 2 + 1) {
      currentText = "Long Break";
    } else {
      if (currentDot % 2 == 0) {
        currentText = "Work";
      } else {
        currentText = "Short Break";
      }
    }

    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: childrenDots,
        ),
        Container(
          margin: const EdgeInsets.only(
            top: 20,
          ),
          child: Text(
            currentText,
            style: const TextStyle(
              fontSize: 30,
            ),
          ),
        ),
      ],
    );
  }
}
