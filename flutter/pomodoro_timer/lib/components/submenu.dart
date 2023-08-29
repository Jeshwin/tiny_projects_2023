import 'package:flutter/material.dart';
import 'package:pomodoro_timer/utils/helper_functions.dart';

class Submenu extends StatefulWidget {
  const Submenu({super.key, required this.title, required this.children});

  final String title;
  final List<Widget> children;

  @override
  State<Submenu> createState() => _SubmenuState();
}

class _SubmenuState extends State<Submenu> {
  @override
  Widget build(BuildContext context) {
    List<Widget> organizedChildren = [];

    for (var i = 0; i < widget.children.length; i++) {
      Widget child = widget.children[i];

      if (i != widget.children.length - 1) {
        organizedChildren.add(Container(
          margin: const EdgeInsets.only(
            bottom: 5,
          ),
          child: child,
        ));
      } else {
        organizedChildren.add(child);
      }
    }
    return Container(
      margin: const EdgeInsets.only(
        left: 40,
        right: 40,
        bottom: 25,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            widget.title,
            style: TextStyle(
              fontSize: 30,
              color: Theme.of(context).primaryColor,
            ),
          ),
          AnimatedContainer(
            duration: const Duration(milliseconds: 700),
            decoration: BoxDecoration(
              color: Theme.of(context).scaffoldBackgroundColor,
              borderRadius: BorderRadius.circular(10),
              boxShadow: [
                BoxShadow(
                  offset: const Offset(5, 10),
                  color: offsetColor(
                      Theme.of(context).scaffoldBackgroundColor, -21),
                  blurRadius: 20,
                ),
                BoxShadow(
                  offset: const Offset(-5, -10),
                  color: offsetColor(
                      Theme.of(context).scaffoldBackgroundColor, 21),
                  blurRadius: 30,
                ),
              ],
              gradient: LinearGradient(
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
                colors: [
                  offsetColor(Theme.of(context).scaffoldBackgroundColor, 7),
                  offsetColor(Theme.of(context).scaffoldBackgroundColor, -7),
                ],
              ),
            ),
            margin: const EdgeInsets.symmetric(vertical: 10),
            padding: const EdgeInsets.symmetric(
              vertical: 10,
              horizontal: 15,
            ),
            width: MediaQuery.of(context).size.width,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: organizedChildren,
            ),
          )
        ],
      ),
    );
  }
}
