import 'dart:async';

import 'package:flutter/material.dart';
import 'package:pomodoro_timer/components/neumorphic_clock.dart';
import 'package:pomodoro_timer/components/pomodoro_dots.dart';
import 'components/neumorphic_button.dart';
import 'utils/constants.dart';

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
        brightness: Brightness.light,
        scaffoldBackgroundColor: const Color(0xffeff1f5),
        primaryColor: const Color(0xff4c4f69),
        fontFamily: 'NunitoSans',
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        scaffoldBackgroundColor: const Color(0xff24273a),
        primaryColor: const Color(0xffcad3f5),
        fontFamily: 'NunitoSans',
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
  final int _numCycles = 3;
  int _currentDot = 0;

  bool _isRunning = false;

  Timer? _timer;
  int _numMillis = WORK_TIME;

  // Start the timer given the current cycle
  void _startTimer() {
    setState(() {
      _isRunning = true;
    });

    _timer = Timer.periodic(const Duration(milliseconds: 1), (timer) {
      setState(() {
        if (_numMillis > 0) {
          _numMillis--;
        } else {
          _isRunning = false;
          _currentDot = (_currentDot + 1) % (_numCycles * 2 + 2);
          _timer?.cancel();

          if (_currentDot == _numCycles * 2 + 1) {
            _numMillis = LONG_BREAK_TIME;
          } else if (_currentDot % 2 == 0) {
            _numMillis = WORK_TIME;
          } else {
            _numMillis = SHORT_BREAK_TIME;
          }
        }
      });
    });
  }

  // Pause the timer
  void _pauseTimer() {
    setState(() {
      _isRunning = false;
    });
    _timer?.cancel();
  }

  // Cancel the timer
  void _cancelTimer() {
    setState(() {
      _numMillis = WORK_TIME;
      _currentDot = 0;
      _isRunning = false;
    });
    _timer?.cancel();
  }

  // Convert seconds into a printable string
  String _secondsToString() {
    return '${(_numMillis / 60000).floor().toString().padLeft(2, '0')}:${((_numMillis / 1000) % 60).floor().toString().padLeft(2, '0')}';
  }

  // Change background color based on session and current brightness
  Color _backgroundColor() {
    if (Theme.of(context).brightness == Brightness.light) {
      if (!_isRunning) {
        return Theme.of(context).scaffoldBackgroundColor;
      } else {
        if (_currentDot == _numCycles * 2 + 1) {
          return const Color(0xff74c7ec);
        } else {
          if (_currentDot % 2 == 0) {
            return const Color(0xfff0c6c6);
          } else {
            return const Color(0xffa6da95);
          }
        }
      }
    } else {
      if (!_isRunning) {
        return Theme.of(context).scaffoldBackgroundColor;
      } else {
        if (_currentDot == _numCycles * 2 + 1) {
          return const Color(0xff04a5e5);
        } else {
          if (_currentDot % 2 == 0) {
            return const Color(0xffdd7878);
          } else {
            return const Color(0xff40a02b);
          }
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: _backgroundColor(),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: <Widget>[
            Container(
              margin: const EdgeInsets.symmetric(
                vertical: 75,
              ),
              alignment: Alignment.center,
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceAround,
                children: [
                  NeumorphicButton(
                    size: 40,
                    icon: Icons.rotate_left_rounded,
                    onPressed: () {
                      _cancelTimer();
                    },
                    color: _backgroundColor(),
                  ),
                  Text(
                    "Pomodoro",
                    style: TextStyle(
                      fontSize: 25,
                      color: Theme.of(context).primaryColor,
                    ),
                  ),
                  NeumorphicButton(
                    size: 40,
                    icon: Icons.settings,
                    onPressed: () {},
                    color: _backgroundColor(),
                  ),
                ],
              ),
            ),
            NeumorphicClock(
              numCycles: _numCycles,
              currentDot: _currentDot,
              numMillis: _numMillis,
              color: _backgroundColor(),
            ),
            Container(
              margin: const EdgeInsets.only(
                top: 25,
              ),
              child: Text(
                _secondsToString(),
                style: TextStyle(
                  fontSize: 100,
                  color: Theme.of(context).primaryColor,
                ),
              ),
            ),
            PomodoroDots(
              numCycles: _numCycles,
              currentDot: _currentDot,
              isRunning: _isRunning,
            ),
            Container(
              margin: const EdgeInsets.symmetric(
                vertical: 100,
              ),
              child: NeumorphicButton(
                size: 80,
                icon: (!_isRunning)
                    ? Icons.play_arrow_rounded
                    : Icons.pause_rounded,
                onPressed: () {
                  if (_isRunning) {
                    _pauseTimer();
                  } else {
                    _startTimer();
                  }
                },
                color: _backgroundColor(),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
