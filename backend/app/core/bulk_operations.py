"""
CSV Import/Export Service for Students

Handles CSV generation, parsing, and validation for bulk student operations.
"""

import csv
import io
from typing import List, Dict, Any, Tuple
from datetime import datetime
import uuid

class BulkOperationService:
    """Service for bulk operations on students (import/export)."""

    # Define CSV columns for students
    STUDENT_COLUMNS = [
        'first_name',
        'last_name',
        'email',
        'class_name',
        'roll_no',
        'admission_no',
        'mobile_numbers',  # JSON-like, comma-separated
        'address',
        'city',
        'state',
        'zip_code',
        'date_of_birth',
        'enrollment_date',
        'is_active',
        'notes',
    ]

    @staticmethod
    def students_to_csv(students: List[Dict[str, Any]]) -> str:
        """
        Convert list of student dictionaries to CSV string.
        
        Args:
            students: List of student dictionaries
            
        Returns:
            CSV string with header and rows
        """
        output = io.StringIO()
        writer = csv.DictWriter(output, fieldnames=BulkOperationService.STUDENT_COLUMNS)
        
        writer.writeheader()
        
        for student in students:
            row = {}
            for col in BulkOperationService.STUDENT_COLUMNS:
                value = student.get(col, '')
                
                # Handle special fields
                if col == 'mobile_numbers' and isinstance(value, list):
                    # Convert list to comma-separated string
                    value = ';'.join(value) if value else ''
                elif col in ['is_active'] and isinstance(value, bool):
                    value = 'Yes' if value else 'No'
                elif col in ['enrollment_date'] and isinstance(value, datetime):
                    value = value.isoformat()
                
                row[col] = value or ''
            
            writer.writerow(row)
        
        return output.getvalue()

    @staticmethod
    def csv_to_students(csv_content: str) -> Tuple[List[Dict[str, Any]], List[str]]:
        """
        Parse CSV content and return student dictionaries and validation errors.
        
        Args:
            csv_content: Raw CSV content as string
            
        Returns:
            Tuple of (valid_students, error_messages)
        """
        students = []
        errors = []
        
        try:
            reader = csv.DictReader(io.StringIO(csv_content))
            
            if not reader.fieldnames:
                return [], ["CSV file is empty"]
            
            # Validate that CSV has required columns
            required_cols = {'first_name', 'last_name', 'email'}
            missing_cols = required_cols - set(reader.fieldnames or [])
            if missing_cols:
                return [], [f"Missing required columns: {', '.join(missing_cols)}"]
            
            for row_num, row in enumerate(reader, start=2):  # Start at 2 (row 1 is header)
                try:
                    student = BulkOperationService._parse_student_row(row, row_num)
                    students.append(student)
                except ValueError as e:
                    errors.append(f"Row {row_num}: {str(e)}")
        
        except Exception as e:
            return [], [f"Failed to parse CSV: {str(e)}"]
        
        return students, errors

    @staticmethod
    def _parse_student_row(row: Dict[str, str], row_num: int) -> Dict[str, Any]:
        """
        Parse a single CSV row into a student dictionary.
        
        Args:
            row: Dictionary of row values from CSV
            row_num: Row number for error messages
            
        Returns:
            Parsed student dictionary
            
        Raises:
            ValueError: If row data is invalid
        """
        # Clean whitespace
        row = {k: v.strip() if isinstance(v, str) else v for k, v in row.items()}
        
        # Validate required fields
        first_name = row.get('first_name', '').strip()
        last_name = row.get('last_name', '').strip()
        email = row.get('email', '').strip()
        
        if not first_name:
            raise ValueError("first_name is required")
        if not last_name:
            raise ValueError("last_name is required")
        if not email:
            raise ValueError("email is required")
        
        # Basic email validation
        if '@' not in email or '.' not in email.split('@')[1]:
            raise ValueError(f"Invalid email format: {email}")
        
        # Parse optional fields
        student = {
            'first_name': first_name,
            'last_name': last_name,
            'email': email,
            'class_name': row.get('class_name', '').strip() or None,
            'roll_no': row.get('roll_no', '').strip() or None,
            'admission_no': row.get('admission_no', '').strip() or None,
            'address': row.get('address', '').strip() or None,
            'city': row.get('city', '').strip() or None,
            'state': row.get('state', '').strip() or None,
            'zip_code': row.get('zip_code', '').strip() or None,
            'date_of_birth': row.get('date_of_birth', '').strip() or None,
            'enrollment_date': row.get('enrollment_date', '').strip() or None,
            'notes': row.get('notes', '').strip() or None,
        }
        
        # Parse mobile numbers (semicolon-separated)
        mobile_str = row.get('mobile_numbers', '').strip()
        if mobile_str:
            student['mobile_numbers'] = [m.strip() for m in mobile_str.split(';') if m.strip()]
        else:
            student['mobile_numbers'] = []
        
        # Parse boolean field
        is_active_str = row.get('is_active', 'Yes').strip().lower()
        student['is_active'] = is_active_str in ['yes', 'true', '1', 'y']
        
        return student

    @staticmethod
    def generate_csv_template() -> str:
        """
        Generate a CSV template with sample data for users to download.
        
        Returns:
            CSV string with header and sample rows
        """
        output = io.StringIO()
        writer = csv.DictWriter(output, fieldnames=BulkOperationService.STUDENT_COLUMNS)
        
        writer.writeheader()
        
        # Sample row 1
        writer.writerow({
            'first_name': 'John',
            'last_name': 'Doe',
            'email': 'john.doe@example.com',
            'class_name': '10-A',
            'roll_no': '101',
            'admission_no': 'ADM-2024-001',
            'mobile_numbers': '9876543210;9876543211',
            'address': '123 Main St',
            'city': 'New York',
            'state': 'NY',
            'zip_code': '10001',
            'date_of_birth': '2008-01-15',
            'enrollment_date': '2023-06-01',
            'is_active': 'Yes',
            'notes': 'Sample student',
        })
        
        # Sample row 2
        writer.writerow({
            'first_name': 'Jane',
            'last_name': 'Smith',
            'email': 'jane.smith@example.com',
            'class_name': '10-B',
            'roll_no': '102',
            'admission_no': 'ADM-2024-002',
            'mobile_numbers': '9876543212',
            'address': '456 Oak Ave',
            'city': 'Los Angeles',
            'state': 'CA',
            'zip_code': '90001',
            'date_of_birth': '2008-03-20',
            'enrollment_date': '2023-06-01',
            'is_active': 'Yes',
            'notes': '',
        })
        
        return output.getvalue()
