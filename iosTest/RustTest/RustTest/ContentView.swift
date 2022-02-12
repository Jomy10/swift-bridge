//
//  ContentView.swift
//  RustTest
//
//  Created by Jonas Everaert on 12/02/2022.
//

import SwiftUI
import package

struct ContentView: View {
    var body: some View {
        Text(get_hello_rust().toString())
            .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
