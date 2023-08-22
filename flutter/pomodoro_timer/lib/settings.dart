import 'package:flutter/material.dart';
import 'package:settings_ui/settings_ui.dart';
import 'styles/catppuccin.dart';

class SettingsPage extends StatelessWidget {
  const SettingsPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Settings")),
      backgroundColor: Catppuccin().base,
      body: SettingsList(
        sections: [
          SettingsSection(
            title: const Text('Theme'),
            tiles: <SettingsTile>[
              SettingsTile.switchTile(
                onToggle: (value) {},
                initialValue: true,
                leading: const Icon(Icons.brightness_4),
                title: const Text('Dark theme'),
              ),
              SettingsTile.switchTile(
                onToggle: (value) {},
                initialValue: true,
                leading: const Icon(Icons.palette),
                title: const Text('Enable custom theme'),
              ),
              SettingsTile.navigation(
                description: const Text("#181926"),
                leading: const Icon(Icons.photo),
                title: const Text('Background color'),
              ),
              SettingsTile.navigation(
                description: const Text("#cad3f5"),
                leading: const Icon(Icons.font_download),
                title: const Text('Text color'),
              ),
              SettingsTile.navigation(
                description: const Text("#a6da95"),
                leading: const Icon(Icons.water_drop),
                title: const Text('Work color'),
              ),
              SettingsTile.navigation(
                description: const Text("#8bd5ca"),
                leading: const Icon(Icons.water_drop),
                title: const Text('Short Break color'),
              ),
              SettingsTile.navigation(
                description: const Text("#c6a0f6"),
                leading: const Icon(Icons.water_drop),
                title: const Text('Long Break color'),
              ),
            ],
          ),
          SettingsSection(
            title: const Text('Functionality'),
            tiles: <SettingsTile>[
              SettingsTile.switchTile(
                onToggle: (value) {},
                initialValue: true,
                leading: const Icon(Icons.notifications),
                title: const Text('Push notifications'),
              ),
              SettingsTile.switchTile(
                onToggle: (value) {},
                initialValue: true,
                leading: const Icon(Icons.volume_up),
                title: const Text('Play sound'),
              ),
              SettingsTile.navigation(
                description: const Text("Ping"),
                leading: const Icon(Icons.photo),
                title: const Text('Sound'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
