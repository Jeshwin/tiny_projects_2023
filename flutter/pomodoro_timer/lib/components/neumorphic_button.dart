import 'package:flutter/material.dart';
import '../styles/catppuccin.dart';

class NeumorphicButton extends StatelessWidget {
  const NeumorphicButton({
    super.key,
    required this.size,
    required this.icon,
    required this.onPressed,
  });

  final double size;
  final IconData icon;
  final void Function() onPressed;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: size,
      width: size,
      decoration: BoxDecoration(
        color: Catppuccin().base,
        shape: BoxShape.circle,
        boxShadow: [
          BoxShadow(
            offset: Offset(size / 40, size / 16),
            color: Catppuccin().neumorphismDark,
            blurRadius: size / 10,
            spreadRadius: 0.5,
          ),
          BoxShadow(
            offset: Offset(size / -40, size / -20),
            color: Catppuccin().neumorphismLight,
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
          color: Catppuccin().text,
        ),
      ),
    );
  }
}
