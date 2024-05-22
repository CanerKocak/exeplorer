import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'providers/balance_provider.dart';
import 'screens/balance_screen.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => BalanceProvider(),
      child: MaterialApp(
        title: 'Windoge98 Meme Coin App',
        theme: ThemeData(
          useMaterial3: true,
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepOrange),
        ),
        home: BalanceScreen(),
      ),
    );
  }
}
