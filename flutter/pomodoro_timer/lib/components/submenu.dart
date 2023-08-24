import 'package:flutter/material.dart';

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
          Container(
            decoration: BoxDecoration(
              color: Theme.of(context).scaffoldBackgroundColor,
              borderRadius: BorderRadius.circular(10),
              boxShadow: [
                const BoxShadow(
                  offset: Offset(5, 10),
                  color: Colors.black38,
                  blurRadius: 20,
                  spreadRadius: 0.5,
                ),
                BoxShadow(
                  offset: const Offset(-5, -10),
                  color: (Theme.of(context).brightness == Brightness.light)
                      ? Colors.white38
                      : Colors.white12,
                  blurRadius: 30,
                  spreadRadius: 0.1,
                ),
              ],
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
