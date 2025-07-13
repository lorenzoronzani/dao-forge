import { Principal } from '@dfinity/principal';

function generateLocalCanisterIdFromCounter(counter) {
    // Your local canisters follow this pattern:
    // [5 chars]-[5 chars]-77774-[5 chars]-cai
    // The actual bytes are much shorter

    // Create 10-byte array for local canister
    const bytes = new Uint8Array(10);

    // Set the subnet identifier (first few bytes)
    bytes[0] = 0x00;
    bytes[1] = 0x00;
    bytes[2] = 0x00;
    bytes[3] = 0x00;
    bytes[4] = 0x00;
    bytes[5] = 0x00;
    bytes[6] = 0x00;
    bytes[7] = 0x01; // Local subnet marker

    // Set counter in last 2 bytes
    bytes[8] = (counter >>> 8) & 0xFF;
    bytes[9] = counter & 0xFF;

    return Principal.fromUint8Array(bytes).toString();
}

// Generate starting from counter 100
for (let i = 100; i <= 107; i++) {
    console.log(`CANISTER_${i} = '${generateLocalCanisterIdFromCounter(i)}'`);
}