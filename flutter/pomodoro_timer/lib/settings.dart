import 'package:flutter/material.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:pomodoro_timer/components/neumorphic_button.dart';
import 'package:pomodoro_timer/components/submenu.dart';
import 'package:pomodoro_timer/components/top_bar.dart';
import 'package:pomodoro_timer/utils/defaults.dart';
import 'package:pomodoro_timer/utils/settings_model.dart';
import 'package:provider/provider.dart';

class Settings extends StatefulWidget {
  const Settings({super.key});

  @override
  State<Settings> createState() => _SettingsState();
}

class _SettingsState extends State<Settings> {
  // Helper function to convert milliseconds to printable minutes value
  String millisToString(int millis) {
    return "${(millis / 60000).floor()} minutes";
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: AnimatedContainer(
        color: Theme.of(context).scaffoldBackgroundColor,
        duration: const Duration(milliseconds: 700),
        alignment: Alignment.center,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            TopBar(
              left: NeumorphicButton(
                size: 40,
                icon: Icons.arrow_back,
                onPressed: () {
                  Navigator.pop(context);
                },
                color: Theme.of(context).scaffoldBackgroundColor,
              ),
              title: "Settings",
            ),
            Submenu(
              title: "Cycles",
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      "Number of Cycles",
                      style: TextStyle(
                        fontSize: 18,
                      ),
                    ),
                    Consumer<SettingsModel>(
                      builder: (context, settings, child) {
                        List<int> numCyclesOptions = [1, 2, 3, 4, 5, 6];
                        List<DropdownMenuEntry<int>> numCyclesEntries = [];
                        for (int option in numCyclesOptions) {
                          numCyclesEntries.add(DropdownMenuEntry<int>(
                            label: "$option",
                            value: option,
                          ));
                        }
                        return DropdownMenu(
                          width: 72,
                          inputDecorationTheme: const InputDecorationTheme(
                            contentPadding: EdgeInsets.all(0),
                          ),
                          menuStyle: MenuStyle(
                            backgroundColor: MaterialStatePropertyAll(
                                Theme.of(context).scaffoldBackgroundColor),
                          ),
                          textStyle: const TextStyle(
                            fontSize: 18,
                          ),
                          onSelected: (int? option) {
                            settings.setNumCycles(option ?? 2);
                          },
                          initialSelection: settings.numCycles,
                          dropdownMenuEntries: numCyclesEntries,
                        );
                      },
                    ),
                  ],
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      "Start Cycles Automatically",
                      style: TextStyle(
                        fontSize: 18,
                      ),
                    ),
                    Consumer<SettingsModel>(
                      builder: (context, settings, child) => PlatformSwitch(
                        value: settings.isAutomatic,
                        onChanged: (bool value) {
                          settings.setIsAutomatic(value);
                        },
                        activeColor:
                            (Theme.of(context).brightness == Brightness.light)
                                ? const Color(0xff74c7ec)
                                : const Color(0xff04a5e5),
                      ),
                    ),
                  ],
                ),
              ],
            ),
            Submenu(
              title: "Duration",
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      "Work Time",
                      style: TextStyle(
                        fontSize: 18,
                      ),
                    ),
                    Consumer<SettingsModel>(
                      builder: (context, settings, child) {
                        List<DropdownMenuEntry<int>> workTimeEntries = [];
                        for (int option in workTimeOptions) {
                          workTimeEntries.add(DropdownMenuEntry<int>(
                            label: millisToString(option),
                            value: option,
                          ));
                        }
                        return DropdownMenu(
                          width: 144,
                          inputDecorationTheme: const InputDecorationTheme(
                            contentPadding: EdgeInsets.all(0),
                          ),
                          menuStyle: MenuStyle(
                            backgroundColor: MaterialStatePropertyAll(
                              Theme.of(context).scaffoldBackgroundColor,
                            ),
                          ),
                          textStyle: const TextStyle(
                            fontSize: 18,
                          ),
                          onSelected: (int? option) {
                            settings.setWorkTime(option ?? defaultWorkTime);
                          },
                          initialSelection: settings.workTime,
                          dropdownMenuEntries: workTimeEntries,
                        );
                      },
                    ),
                  ],
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      "Short Break",
                      style: TextStyle(
                        fontSize: 18,
                      ),
                    ),
                    Consumer<SettingsModel>(
                      builder: (context, settings, child) {
                        List<DropdownMenuEntry<int>> shortBreakTimeEntries = [];
                        for (int option in shortBreakTimeOptions) {
                          shortBreakTimeEntries.add(DropdownMenuEntry<int>(
                            label: millisToString(option),
                            value: option,
                          ));
                        }
                        return DropdownMenu(
                          width: 144,
                          inputDecorationTheme: const InputDecorationTheme(
                            contentPadding: EdgeInsets.all(0),
                          ),
                          menuStyle: MenuStyle(
                            backgroundColor: MaterialStatePropertyAll(
                              Theme.of(context).scaffoldBackgroundColor,
                            ),
                          ),
                          textStyle: const TextStyle(
                            fontSize: 18,
                          ),
                          onSelected: (int? option) {
                            settings.setShortBreakTIme(
                                option ?? defaultShortBreakTime);
                          },
                          initialSelection: settings.shortBreakTime,
                          dropdownMenuEntries: shortBreakTimeEntries,
                        );
                      },
                    ),
                  ],
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      "Long Break",
                      style: TextStyle(
                        fontSize: 18,
                      ),
                    ),
                    Consumer<SettingsModel>(
                      builder: (context, settings, child) {
                        List<DropdownMenuEntry<int>> longBreakTimeEntries = [];
                        for (int option in longBreakTimeOptions) {
                          longBreakTimeEntries.add(DropdownMenuEntry<int>(
                            label: millisToString(option),
                            value: option,
                          ));
                        }
                        return DropdownMenu(
                          width: 144,
                          inputDecorationTheme: const InputDecorationTheme(
                            contentPadding: EdgeInsets.all(0),
                          ),
                          menuStyle: MenuStyle(
                            backgroundColor: MaterialStatePropertyAll(
                              Theme.of(context).scaffoldBackgroundColor,
                            ),
                          ),
                          textStyle: const TextStyle(
                            fontSize: 18,
                          ),
                          onSelected: (int? option) {
                            settings.setLongBreakTime(
                                option ?? defaultLongBreakTime);
                          },
                          initialSelection: settings.longBreakTime,
                          dropdownMenuEntries: longBreakTimeEntries,
                        );
                      },
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
