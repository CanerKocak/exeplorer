import 'package:flutter/material.dart';
import '../models/canister_response.dart';
import '../utils/agent_helper.dart';

class BalanceProvider extends ChangeNotifier {
  CanisterResponse? _balance;
  bool _loading = false;

  CanisterResponse? get balance => _balance;
  bool get loading => _loading;

  Future<void> fetchBalance(String principalId) async {
    _loading = true;
    notifyListeners();

    try {
      final response = await AgentHelper.getBalance(principalId);
      _balance = CanisterResponse(balance: response);
    } catch (e) {
      print('Error fetching balance: $e');
    }

    _loading = false;
    notifyListeners();
  }
}
