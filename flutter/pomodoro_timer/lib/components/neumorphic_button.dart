import 'package:flutter/material.dart';

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
    return Container(
      height: size,
      width: size,
      decoration: BoxDecoration(
        color: color,
        shape: BoxShape.circle,
        boxShadow: [
          BoxShadow(
            offset: Offset(size / 40, size / 16),
            color: Colors.black38,
            blurRadius: size / 10,
            spreadRadius: 0.5,
          ),
          BoxShadow(
            offset: Offset(size / -40, size / -20),
            color: (Theme.of(context).brightness == Brightness.light)
                ? Colors.white38
                : Colors.white12,
            blurRadius: size / 6.67,
            spreadRadius: 0.1,
          ),
        ],
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
