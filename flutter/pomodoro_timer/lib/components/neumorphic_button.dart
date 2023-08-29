import 'package:flutter/material.dart';
import 'package:pomodoro_timer/utils/helper_functions.dart';

class NeumorphicButton extends StatelessWidget {
  const NeumorphicButton({
    super.key,
    required this.size,
    required this.icon,
    required this.onPressed,
    required this.color,
  });

  final double size;
  final IconData icon;
  final void Function() onPressed;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      height: size,
      width: size,
      duration: const Duration(milliseconds: 700),
      decoration: BoxDecoration(
        color: color,
        shape: BoxShape.circle,
        boxShadow: [
          BoxShadow(
            offset: Offset(size / 40, size / 16),
            color: offsetColor(color, -21),
            blurRadius: size / 10,
          ),
          BoxShadow(
            offset: Offset(size / -40, size / -20),
            color: offsetColor(color, 21),
            blurRadius: size / 6.67,
          ),
        ],
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            offsetColor(color, -7),
            offsetColor(color, 7),
          ],
        ),
      ),
      child: IconButton(
        onPressed: onPressed,
        icon: Icon(
          icon,
          size: size / 1.6,
          color: Theme.of(context).primaryColor,
        ),
      ),
    );
  }
}
