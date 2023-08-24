import 'package:flutter/material.dart';

class TopBar extends StatefulWidget {
  const TopBar({
    super.key,
    this.left = const SizedBox(
      width: 40,
      height: 40,
    ),
    required this.title,
    this.right = const SizedBox(
      width: 40,
      height: 40,
    ),
  });

  final Widget left;
  final String title;
  final Widget right;

  @override
  State<TopBar> createState() => _TopBarState();
}

class _TopBarState extends State<TopBar> {
  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(
        top: 75,
        bottom: 50,
      ),
      alignment: Alignment.center,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceAround,
        children: [
          widget.left,
          Text(
            widget.title,
            style: TextStyle(
              fontSize: 25,
              color: Theme.of(context).primaryColor,
            ),
          ),
          widget.right
        ],
      ),
    );
  }
}
