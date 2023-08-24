import 'package:flutter/material.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:pomodoro_timer/components/neumorphic_button.dart';
import 'package:pomodoro_timer/utils/settings_enums.dart';
import 'package:shared_preferences/shared_preferences.dart';

class Settings extends StatefulWidget {
  const Settings({super.key});

  @override
  State<Settings> createState() => _SettingsState();
}

class _SettingsState extends State<Settings> {
  bool _automaticCyclesToggle = false;

  Future<void> setNumCycles(int numCycles) async {
    final prefs = await SharedPreferences.getInstance();

    await prefs.setInt('numCycles', numCycles);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        color: Theme.of(context).scaffoldBackgroundColor,
        alignment: Alignment.center,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Container(
              margin: const EdgeInsets.only(
                top: 75,
              ),
              alignment: Alignment.center,
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceAround,
                children: [
                  NeumorphicButton(
                    size: 40,
                    icon: Icons.arrow_back,
                    onPressed: () {
                      Navigator.pop(context);
                    },
                    color: Theme.of(context).scaffoldBackgroundColor,
                  ),
                  Text(
                    "Settings",
                    style: TextStyle(
                      fontSize: 25,
                      color: Theme.of(context).primaryColor,
                    ),
                  ),
                  const SizedBox(
                    width: 40,
                    height: 40,
                  ),
                ],
              ),
            ),
            Container(
              margin: const EdgeInsets.only(
                top: 25,
                left: 40,
                right: 40,
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    "Cycles",
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
                          color:
                              (Theme.of(context).brightness == Brightness.light)
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
                      children: [
                        Container(
                          margin: const EdgeInsets.only(
                            bottom: 5,
                          ),
                          child: Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              const Text(
                                "Number of Cycles",
                                style: TextStyle(
                                  fontSize: 18,
                                ),
                              ),
                              DropdownMenu(
                                width: 72,
                                inputDecorationTheme:
                                    const InputDecorationTheme(
                                  contentPadding: EdgeInsets.all(0),
                                ),
                                menuStyle: MenuStyle(
                                  backgroundColor: MaterialStatePropertyAll(
                                      Theme.of(context)
                                          .scaffoldBackgroundColor),
                                ),
                                textStyle: const TextStyle(
                                  fontSize: 18,
                                ),
                                onSelected: (numCyclesOptions? option) {
                                  setNumCycles(option?.numCycles ?? 2);
                                },
                                initialSelection: numCyclesOptions.two,
                                dropdownMenuEntries: [
                                  DropdownMenuEntry<numCyclesOptions>(
                                    label: numCyclesOptions.one.label,
                                    value: numCyclesOptions.one,
                                  ),
                                  DropdownMenuEntry<numCyclesOptions>(
                                    label: numCyclesOptions.two.label,
                                    value: numCyclesOptions.two,
                                  ),
                                  DropdownMenuEntry<numCyclesOptions>(
                                    label: numCyclesOptions.three.label,
                                    value: numCyclesOptions.three,
                                  ),
                                  DropdownMenuEntry<numCyclesOptions>(
                                    label: numCyclesOptions.four.label,
                                    value: numCyclesOptions.four,
                                  ),
                                  DropdownMenuEntry<numCyclesOptions>(
                                    label: numCyclesOptions.five.label,
                                    value: numCyclesOptions.five,
                                  ),
                                  DropdownMenuEntry<numCyclesOptions>(
                                    label: numCyclesOptions.six.label,
                                    value: numCyclesOptions.six,
                                  ),
                                ],
                              ),
                            ],
                          ),
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
                            PlatformSwitch(
                              value: _automaticCyclesToggle,
                              onChanged: (bool value) {
                                setState(() {
                                  _automaticCyclesToggle = value;
                                });
                              },
                            ),
                          ],
                        ),
                      ],
                    ),
                  )
                ],
              ),
            ),
            Container(
              margin: const EdgeInsets.only(
                top: 25,
                left: 40,
                right: 40,
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    "Durations",
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
                          offset: Offset(5, 12),
                          color: Colors.black38,
                          blurRadius: 20,
                          spreadRadius: 0.5,
                        ),
                        BoxShadow(
                          offset: const Offset(-5, -10),
                          color:
                              (Theme.of(context).brightness == Brightness.light)
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
                      children: [
                        Container(
                          margin: const EdgeInsets.only(
                            bottom: 5,
                          ),
                          child: Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              const Text(
                                "Work Time",
                                style: TextStyle(
                                  fontSize: 18,
                                ),
                              ),
                              DropdownMenu(
                                width: 144,
                                inputDecorationTheme:
                                    const InputDecorationTheme(
                                  contentPadding: EdgeInsets.all(0),
                                ),
                                menuStyle: MenuStyle(
                                  backgroundColor: MaterialStatePropertyAll(
                                      Theme.of(context)
                                          .scaffoldBackgroundColor),
                                ),
                                textStyle: const TextStyle(
                                  fontSize: 18,
                                ),
                                initialSelection: workTimerOptions.twentyfive,
                                dropdownMenuEntries: [
                                  DropdownMenuEntry<workTimerOptions>(
                                    label: workTimerOptions.fifteen.label,
                                    value: workTimerOptions.fifteen,
                                  ),
                                  DropdownMenuEntry<workTimerOptions>(
                                    label: workTimerOptions.twenty.label,
                                    value: workTimerOptions.twenty,
                                  ),
                                  DropdownMenuEntry<workTimerOptions>(
                                    label: workTimerOptions.twentyfive.label,
                                    value: workTimerOptions.twentyfive,
                                  ),
                                  DropdownMenuEntry<workTimerOptions>(
                                    label: workTimerOptions.thirty.label,
                                    value: workTimerOptions.thirty,
                                  ),
                                  DropdownMenuEntry<workTimerOptions>(
                                    label: workTimerOptions.fourtyfive.label,
                                    value: workTimerOptions.fourtyfive,
                                  ),
                                  DropdownMenuEntry<workTimerOptions>(
                                    label: workTimerOptions.fifty.label,
                                    value: workTimerOptions.fifty,
                                  ),
                                  DropdownMenuEntry<workTimerOptions>(
                                    label: workTimerOptions.sixty.label,
                                    value: workTimerOptions.sixty,
                                  ),
                                ],
                              ),
                            ],
                          ),
                        ),
                        Container(
                          margin: const EdgeInsets.only(
                            bottom: 5,
                          ),
                          child: Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              const Text(
                                "Short Break",
                                style: TextStyle(
                                  fontSize: 18,
                                ),
                              ),
                              DropdownMenu(
                                width: 144,
                                inputDecorationTheme:
                                    const InputDecorationTheme(
                                  contentPadding: EdgeInsets.all(0),
                                ),
                                menuStyle: MenuStyle(
                                  backgroundColor: MaterialStatePropertyAll(
                                      Theme.of(context)
                                          .scaffoldBackgroundColor),
                                ),
                                textStyle: const TextStyle(
                                  fontSize: 18,
                                ),
                                initialSelection: shortBreakTimerOptions.five,
                                dropdownMenuEntries: [
                                  DropdownMenuEntry<shortBreakTimerOptions>(
                                    label: shortBreakTimerOptions.five.label,
                                    value: shortBreakTimerOptions.five,
                                  ),
                                  DropdownMenuEntry<shortBreakTimerOptions>(
                                    label: shortBreakTimerOptions.ten.label,
                                    value: shortBreakTimerOptions.ten,
                                  ),
                                ],
                              ),
                            ],
                          ),
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
                            DropdownMenu(
                              width: 144,
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
                              initialSelection: longBreakTimerOptions.fifteen,
                              dropdownMenuEntries: [
                                DropdownMenuEntry<longBreakTimerOptions>(
                                  label: longBreakTimerOptions.fifteen.label,
                                  value: longBreakTimerOptions.fifteen,
                                ),
                                DropdownMenuEntry<longBreakTimerOptions>(
                                  label: longBreakTimerOptions.twenty.label,
                                  value: longBreakTimerOptions.twenty,
                                ),
                                DropdownMenuEntry<longBreakTimerOptions>(
                                  label: longBreakTimerOptions.thirty.label,
                                  value: longBreakTimerOptions.thirty,
                                ),
                              ],
                            ),
                          ],
                        ),
                      ],
                    ),
                  )
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
