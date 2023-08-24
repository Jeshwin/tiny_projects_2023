import 'package:flutter/foundation.dart';
import 'package:pomodoro_timer/utils/defaults.dart';
import 'package:shared_preferences/shared_preferences.dart';

class SettingsModel extends ChangeNotifier {
  // Cycles settings
  int _numCycles = 2;
  bool _isAutomatic = false;

  int get numCycles => _numCycles;
  bool get isAutomatic => _isAutomatic;

  Future<void> setNumCycles(int value) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setInt('numCycles', value);
    _numCycles = value;
    notifyListeners();
  }

  Future<void> setIsAutomatic(bool value) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setBool('isAutomatic', value);
    _isAutomatic = value;
    notifyListeners();
  }

  // Duration settings
  int _workTime = defaultWorkTime;
  int _shortBreakTIme = defaultShortBreakTime;
  int _longBreakTime = defaultLongBreakTime;

  int get workTime => _workTime;
  int get shortBreakTime => _shortBreakTIme;
  int get longBreakTime => _longBreakTime;

  Future<void> setWorkTime(int value) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setInt('workTime', value);
    _workTime = value;
    notifyListeners();
  }

  Future<void> setShortBreakTIme(int value) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setInt('shortBreakTIme', value);
    _shortBreakTIme = value;
    notifyListeners();
  }

  Future<void> setLongBreakTime(int value) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setInt('longBreakTime', value);
    _longBreakTime = value;
    notifyListeners();
  }

  SettingsModel() {
    setup();
  }

  Future<void> setup() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();

    setNumCycles(prefs.getInt('numCycles') ?? 2);
    setIsAutomatic(prefs.getBool('isAutomatic') ?? false);

    setWorkTime(prefs.getInt('workTime') ?? defaultWorkTime);
    setShortBreakTIme(prefs.getInt('shortBreakTime') ?? defaultShortBreakTime);
    setLongBreakTime(prefs.getInt('longBreakTIme') ?? defaultLongBreakTime);
  }
}
