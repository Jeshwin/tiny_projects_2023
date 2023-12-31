import 'dart:async';

import 'package:flutter/material.dart';
import 'package:pomodoro_timer/components/clock/neumorphic_clock.dart';
import 'package:pomodoro_timer/components/pomodoro_dots.dart';
import 'package:pomodoro_timer/components/top_bar.dart';
import 'package:pomodoro_timer/settings.dart';
import 'package:pomodoro_timer/utils/notification_service.dart';
import 'package:pomodoro_timer/utils/settings_model.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'components/neumorphic_button.dart';
import 'utils/defaults.dart';

void main() {
  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(
          create: (context) => SettingsModel(),
        ),
      ],
      child: const MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Pomodoro',
      theme: ThemeData(
        brightness: Brightness.light,
        scaffoldBackgroundColor: const Color(0xffeff1f5),
        fontFamily: 'NunitoSans',
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        scaffoldBackgroundColor: const Color(0xff24273a),
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
  int _numCycles = 2;
  int _currentDot = 0;

  Timer? _timer;
  bool _isRunning = false;
  bool _automaticCycles = false;

  int _numMillis = defaultWorkTime;
  int _workTime = defaultWorkTime;
  int _shortBreakTime = defaultShortBreakTime;
  int _longBreakTime = defaultLongBreakTime;

  int _notificationId = 0;

  @override
  void initState() {
    super.initState();
    NotificationService().initNotification();
    _loadTimers();
  }

  Future<void> _loadTimers() async {
    // obtain shared preferences
    final prefs = await SharedPreferences.getInstance();

    setState(() {
      _pauseTimer();
      _automaticCycles = prefs.getBool('isAutomatic') ?? false;
      _numCycles = prefs.getInt('numCycles') ?? 2;

      _workTime = prefs.getInt('workTime') ?? defaultWorkTime;
      _shortBreakTime = prefs.getInt('shortBreakTime') ?? defaultShortBreakTime;
      _longBreakTime = prefs.getInt('longBreakTIme') ?? defaultLongBreakTime;
      _numMillis = prefs.getInt('workTime') ?? defaultWorkTime;
    });
  }

  // Start the timer given the current cycle
  void _startTimer() {
    setState(() {
      _isRunning = true;
    });

    _timer = Timer.periodic(const Duration(milliseconds: 5), (timer) {
      setState(() {
        if (_numMillis > 0) {
          _numMillis -= 5;
        } else {
          _isRunning = false;
          _currentDot = (_currentDot + 1) % (_numCycles * 2 + 2);
          _timer?.cancel();

          if (_currentDot == _numCycles * 2 + 1) {
            NotificationService().showNotification(
              id: _notificationId,
              title: "Take a break",
              body: "Nice work! Let's take a long break!",
            );
            _numMillis = _longBreakTime;
          } else if (_currentDot % 2 == 0) {
            NotificationService().showNotification(
              id: _notificationId,
              title: "Time to work",
              body: "Let's get back to work!",
            );
            _numMillis = _workTime;
          } else {
            NotificationService().showNotification(
              id: _notificationId,
              title: "Take a short break",
              body: "Rest for a bit, then come back later!",
            );
            _numMillis = _shortBreakTime;
          }

          _notificationId = (_notificationId + 1) % (_numCycles * 2 + 2);

          if (_automaticCycles) {
            _startTimer();
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
      _numMillis = _workTime;
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

  FutureOr onGoBack(dynamic value) {
    _loadTimers();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: AnimatedContainer(
        color: _backgroundColor(),
        duration: const Duration(milliseconds: 700),
        alignment: Alignment.center,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          children: <Widget>[
            TopBar(
              left: NeumorphicButton(
                size: 40,
                icon: Icons.rotate_left_rounded,
                onPressed: () {
                  _cancelTimer();
                },
                color: _backgroundColor(),
              ),
              title: "Pomodoro",
              right: NeumorphicButton(
                size: 40,
                icon: Icons.settings,
                onPressed: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const Settings()),
                  ).then(onGoBack);
                },
                color: _backgroundColor(),
              ),
            ),
            NeumorphicClock(
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
                style: const TextStyle(
                  fontSize: 100,
                ),
              ),
            ),
            PomodoroDots(
              numCycles: _numCycles,
              currentDot: _currentDot,
              isRunning: _isRunning,
            ),
            Container(
              margin: const EdgeInsets.only(
                top: 50,
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
