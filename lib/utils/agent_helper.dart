import 'package:agent_dart/agent_dart.dart';

class AgentHelper {
  static final agent = Agent(options: const HttpAgentOptions(host: 'https://ic0.app'));

  static Future<BigInt> getBalance(String principalId) async {
    await agent.fetchRootKey(); // For development only
    final canisterId = Principal.fromText('rh2pm-ryaaa-aaaan-qeniq-cai');
    final actor = Actor.createActor(
      IDLFactory.create({
        'balance_of': IDL.Func([IDL.Principal], [IDL.Nat], [])
      }),
      canisterId: canisterId,
      agent: agent,
    );

    final principal = Principal.fromText(principalId);
    final balance = await actor.invoke('balance_of', [principal]);
    return balance[0] as BigInt;
  }
}
