import 'package:flutter/material.dart';
import 'package:web3_babyjubjub_sdk/src/rust/api/simple.dart';
import 'package:web3_babyjubjub_sdk/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  void test() {
    hashPoseidonTest();
  }

  void signPoseidonTest() {
    var res = signPoseidon(privateKey: "32eaa6373fc102f8a1d88a8a264c26c50b22f9de2f128a9c84bfd3b3b1244980", msg: "8361061397615012257162160668573608797973224669051542553979298255625142692807");
    var valid = "124a9472f9ec59e86c49a762d5889c2c73ef6380765c067722e32f8cfce44a09dd94f8322fd65bd946c0e3cc70de6166ffd8d008fb44b06ebfb1ad1eb2e1b404" == res;
    print("signPoseidon: $valid");
  }

  void getBJJKeyTest() {
    var res = prv2Pub(privateKey: "32eaa6373fc102f8a1d88a8a264c26c50b22f9de2f128a9c84bfd3b3b1244980");
    var valid = "0x25ac4be6256811aadcaa311efabf2e286885b4da2070320c3becfc5fa6c5733b" == res;
    print(res);
    print("getBJJKeyTest: $valid");
  }

  void packPointTest() async {
    var p1 = "5067058882184289685879291240436517726527787201084588250492822232261202434720";
    var p2 = "17039996928425847512124231608079985051815305514092976608073695914635437634363";
    var res = packPoint(pointX: p1, pointY: p2);
    print(res);
  }

  void hashPoseidonTest() async {
    var msg = "62771017353866807638606167115890313645268136916697939166887948";
    var res = hashPoseidon(txCompressedData: msg);
    print(res);
  }


  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: ElevatedButton(
            child: Text("test"),
            onPressed: () => test(),
              ),
        ),
      ),
    );
  }
}
