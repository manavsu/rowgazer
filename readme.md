# Architecture

1. **Frontend (UI with `iced`)**
   - **Main Window**: Displays the CSV data in a scrollable table.
   - **File Selector**: Allows users to select a CSV file to load.
   - **Search/Filter Bar**: Enables filtering rows based on user input.

2. **Backend (Data Handling)**
   - **CSV Parser**: Reads and parses CSV files into a structured format (e.g., `Vec<Vec<String>>`).
   - **Data Store**: Manages the parsed data and provides filtered views.
   - **Search/Filter Logic**: Filters rows based on user input.

3. **State Management**
   - **App State**: Tracks the current file, parsed data, and filter state.
   - **Message Handling**: Handles user interactions (e.g., file selection, search input).

4. **Error Handling**
   - **File Errors**: Handles invalid or unreadable CSV files.
   - **UI Errors**: Displays user-friendly error messages.
