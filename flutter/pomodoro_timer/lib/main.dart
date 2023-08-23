import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:pomodoro_timer/components/pomodoro_dots.dart';
import 'styles/catppuccin.dart';
import 'components/neumorphic_button.dart';
import 'settings.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: const ColorScheme.light(),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        colorScheme: const ColorScheme.dark(),
        useMaterial3: true,
      ),
      home: const MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Catppuccin().base,
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: <Widget>[
            Container(
              margin: const EdgeInsets.symmetric(
                vertical: 40,
              ),
              alignment: Alignment.center,
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceAround,
                children: [
                  const SizedBox(
                    width: 40,
                    height: 40,
                  ),
                  Text(
                    "Pomodoro",
                    style: GoogleFonts.rubik(
                      textStyle: TextStyle(
                        fontSize: 25,
                        color: Catppuccin().text,
                      ),
                    ),
                  ),
                  NeumorphicButton(
                    size: 40,
                    icon: Icons.settings,
                    onPressed: () {
                      Navigator.of(context).push(
                        MaterialPageRoute(
                          builder: (context) => const SettingsPage(),
                        ),
                      );
                    },
                  ),
                ],
              ),
            ),
            Container(
              margin: const EdgeInsets.only(
                top: 150,
              ),
              child: Text(
                "25:00",
                style: GoogleFonts.rubik(
                  textStyle: TextStyle(
                    fontSize: 100,
                    color: Catppuccin().text,
                  ),
                ),
              ),
            ),
            const PomodoroDots(
              numCycles: 3,
              currentDot: 1,
            ),
            Container(
              margin: const EdgeInsets.symmetric(
                vertical: 120,
              ),
              child: NeumorphicButton(
                size: 80,
                icon: Icons.play_arrow_rounded,
                onPressed: () {},
              ),
            ),
          ],
        ),
      ),
    );
  }
}
