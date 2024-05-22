import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/balance_provider.dart';
import '../widgets/loading_widget.dart';

class BalanceScreen extends StatelessWidget {
  final TextEditingController _controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Check Balance')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            TextField(
              controller: _controller,
              decoration: InputDecoration(labelText: 'Enter Principal ID'),
            ),
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: () {
                final provider = Provider.of<BalanceProvider>(context, listen: false);
                provider.fetchBalance(_controller.text);
              },
              child: Text('Check Balance'),
            ),
            SizedBox(height: 20),
            Consumer<BalanceProvider>(
              builder: (context, provider, child) {
                if (provider.loading) {
                  return LoadingWidget();
                } else if (provider.balance != null) {
                  return Text('Balance: ${provider.balance!.balance}');
                } else {
                  return Text('Enter a valid Principal ID to check balance.');
                }
              },
            ),
          ],
        ),
      ),
    );
  }
}
