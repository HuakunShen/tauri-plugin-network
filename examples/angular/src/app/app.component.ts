import { Component } from "@angular/core";
import { CommonModule } from '@angular/common';
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { ChangeDetectorRef } from '@angular/core';
import { nonLocalhostNetworks } from "tauri-plugin-network-api";

@Component({
  selector: "app-root",
  imports: [RouterOutlet, CommonModule],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  greetingMessage = "";
  ips: String[] = [];

  constructor(private cdr: ChangeDetectorRef) {}

  async findIps(event: SubmitEvent): Promise<void> {
    event.preventDefault();

    const nets = await nonLocalhostNetworks();
    nets.forEach(net => {
      this.ips.push(net.addr); 
    });

    this.cdr.detectChanges(); // forces view update
  }

  // Default interact with rust example code
  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }
}
