<script setup lang="ts">
const props = defineProps({
    modelValue: {
        type: Date,
        default: () => new Date(),
    },
    format: {
        type: String,
        default: 'YYYY-MM-DD',
    },
})

const withTime = computed(
    () => props.format.includes('HH') || props.format.includes('mm')
)

const emits = defineEmits(['update:modelValue'])

const pickerOpen = ref(false)
const pickerRef = ref<HTMLElement | null>(null)
const buttonRef = ref<HTMLElement | null>(null)
const pickerView = ref<'days' | 'months' | 'years'>('days')

const weekdays = ['S', 'M', 'T', 'W', 'T', 'F', 'S']
const monthNames = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
]
const monthAbbreviations = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
]

const selectedDate = ref(new Date(props.modelValue))
if (!withTime.value) {
    selectedDate.value.setHours(0, 0, 0, 0)
    emits('update:modelValue', selectedDate.value)
}
const selectedTime = ref(formatDate(selectedDate.value, 'HH:mm'))

// Watch for changes in selectedDate and update selectedTime
watch(selectedDate, (newValue) => {
    selectedTime.value = formatDate(newValue, 'HH:mm')
    emits('update:modelValue', newValue)
})

// Watch for changes in selectedTime and update selectedDate
watch(selectedTime, (newValue) => {
    const [hours, minutes] = newValue.split(':').map(Number)
    const date = new Date(selectedDate.value)
    date.setHours(hours)
    date.setMinutes(minutes)
    date.setSeconds(0)
    date.setMilliseconds(0)
    selectedDate.value = date
})

// Compute an array of days in the selected month/year
const daysInMonth = computed(() => {
    const days = []
    const year = selectedDate.value.getFullYear()
    const month = selectedDate.value.getMonth()

    const lastDayOfCurrentMonth = new Date(year, month + 1, 0).getDate()
    const firstDayOfWeek = new Date(year, month, 1).getDay()
    const lastDayOfPreviousMonth = new Date(year, month, 0).getDate()

    // Previous month's days
    for (let i = firstDayOfWeek - 1; i >= 0; i--) {
        const prevMonth = month - 1 < 0 ? 11 : month - 1
        const prevYear = month - 1 < 0 ? year - 1 : year
        days.push({
            day: lastDayOfPreviousMonth - i,
            month: prevMonth,
            year: prevYear,
            currentMonth: false,
        })
    }

    // Current month's days
    for (let i = 1; i <= lastDayOfCurrentMonth; i++) {
        days.push({ day: i, month: month, year: year, currentMonth: true })
    }

    // Next month's days
    let nextMonth = month + 1 > 11 ? 0 : month + 1
    let nextYear = month + 1 > 11 ? year + 1 : year
    let dayCounter = 1
    while (days.length % 7 !== 0) {
        days.push({
            day: dayCounter++,
            month: nextMonth,
            year: nextYear,
            currentMonth: false,
        })
    }

    return days
})

function toggleSelector() {
    pickerOpen.value = !pickerOpen.value
    pickerView.value = 'days'

    if (pickerOpen.value) {
        nextTick(() => {
            positionPicker()
        })
    }
}

function positionPicker() {
    if (pickerRef.value && buttonRef.value) {
        const buttonRect = buttonRef.value.getBoundingClientRect()
        const pickerEl = pickerRef.value

        const pickerHeight = pickerEl.offsetHeight
        const pickerWidth = pickerEl.offsetWidth

        const viewportHeight = window.innerHeight
        const viewportWidth = window.innerWidth

        // Calculate positions
        let top = buttonRect.bottom + window.scrollY + 4
        let left =
            buttonRect.left +
            window.scrollX +
            buttonRect.width / 2 -
            pickerWidth / 2

        // Adjust positioning if not enough space
        if (left < 0) {
            left = 4 // Add some padding from the left edge
        } else if (left + pickerWidth > viewportWidth) {
            left = viewportWidth - pickerWidth - 4 // Add some padding from the right edge
        }

        if (top + pickerHeight > viewportHeight) {
            // Not enough space below, position above the button
            top = buttonRect.top + window.scrollY - pickerHeight - 4
        }

        // Apply positioning styles
        pickerEl.style.top = `${top}px`
        pickerEl.style.left = `${left}px`
    }
}

function selectDate(day: { day: number; month: number; year: number }) {
    // Preserve the existing time components
    const currentHours = selectedDate.value.getHours()
    const currentMinutes = selectedDate.value.getMinutes()
    const currentSeconds = selectedDate.value.getSeconds()
    const currentMilliseconds = selectedDate.value.getMilliseconds()

    // Create a new Date object with the preserved time components
    selectedDate.value = new Date(
        day.year,
        day.month,
        day.day,
        currentHours,
        currentMinutes,
        currentSeconds,
        currentMilliseconds
    )
    pickerOpen.value = false
}

function isSelectedDay(day: {
    day: number
    month: number
    year: number
}): boolean {
    return (
        selectedDate.value.getDate() === day.day &&
        selectedDate.value.getMonth() === day.month &&
        selectedDate.value.getFullYear() === day.year
    )
}

function formatDate(date: Date, format: string): string {
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0') // getMonth() is zero-indexed
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')
    const seconds = String(date.getSeconds()).padStart(2, '0')

    return format
        .replace(/YYYY/g, year.toString())
        .replace(/MMMM/g, monthNames[date.getMonth()])
        .replace(/MMM/g, monthAbbreviations[date.getMonth()])
        .replace(/MM/g, month)
        .replace(/DD/g, day)
        .replace(/HH/g, hours)
        .replace(/mm/g, minutes)
        .replace(/ss/g, seconds)
}

function offsetDate(offset: number) {
    const currentYear = selectedDate.value.getFullYear()
    const currentMonth = selectedDate.value.getMonth()
    const currentDay = selectedDate.value.getDate()

    if (pickerView.value === 'years') {
        const shift = offset < 0 ? -12 : 12
        changeDateYear(currentYear + shift, false)
        return
    }

    const newDate = new Date(currentYear, currentMonth + offset, 1)
    const newYear = newDate.getFullYear()
    const newMonth = newDate.getMonth()
    const lastDayOfNewMonth = new Date(newYear, newMonth + 1, 0).getDate()

    const newDay = Math.min(currentDay, lastDayOfNewMonth)

    selectedDate.value = new Date(
        newYear,
        newMonth,
        newDay,
        selectedDate.value.getHours(),
        selectedDate.value.getMinutes(),
        0,
        0
    )
}

function changeDateMonth(newMonth: number) {
    const currentYear = selectedDate.value.getFullYear()
    const currentDay = selectedDate.value.getDate()
    const lastDayOfNewMonth = new Date(currentYear, newMonth + 1, 0).getDate()

    const newDay = Math.min(currentDay, lastDayOfNewMonth)

    selectedDate.value = new Date(
        currentYear,
        newMonth,
        newDay,
        selectedDate.value.getHours(),
        selectedDate.value.getMinutes(),
        0,
        0
    )
    pickerView.value = 'days'
}

function getYearsArray() {
    const currentYear = selectedDate.value.getFullYear()
    const startYear = currentYear - 7
    return Array.from({ length: 12 }, (_, i) => startYear + i)
}

function changeDateYear(newYear: number, closeYearView = true) {
    const currentMonth = selectedDate.value.getMonth()
    const currentDay = selectedDate.value.getDate()
    const lastDayOfNewMonth = new Date(newYear, currentMonth + 1, 0).getDate()

    const newDay = Math.min(currentDay, lastDayOfNewMonth)

    selectedDate.value = new Date(
        newYear,
        currentMonth,
        newDay,
        selectedDate.value.getHours(),
        selectedDate.value.getMinutes(),
        0,
        0
    )

    if (closeYearView) {
        pickerView.value = 'days'
    }
}

function handleClickOutside(event: MouseEvent) {
    if (
        pickerRef.value &&
        buttonRef.value &&
        !pickerRef.value.contains(event.target as Node) &&
        !buttonRef.value.contains(event.target as Node)
    ) {
        pickerOpen.value = false
    }
}

function handleKeyDown(event: KeyboardEvent) {
    if (pickerOpen.value) {
        if (event.key === 'Escape') {
            pickerOpen.value = false
        } else if (event.key === 'ArrowLeft') {
            offsetDate(-1)
        } else if (event.key === 'ArrowRight') {
            offsetDate(1)
        }
    } else {
        if (event.key === 'Enter' || event.key === 'ArrowDown') {
            toggleSelector()
        }
    }
}

watch(pickerOpen, (isOpen) => {
    if (isOpen) {
        document.addEventListener('click', handleClickOutside)
        document.addEventListener('keydown', handleKeyDown)
    } else {
        document.removeEventListener('click', handleClickOutside)
        document.removeEventListener('keydown', handleKeyDown)
    }
})

onBeforeUnmount(() => {
    document.removeEventListener('click', handleClickOutside)
    document.removeEventListener('keydown', handleKeyDown)
})
</script>

<template>
    <div class="inline-flex cursor-default flex-col items-center">
        <!-- Date Picker Popover -->
        <div
            v-if="pickerOpen"
            ref="pickerRef"
            class="min-w-60 scale-95 transform rounded-md border border-surface-300 bg-form-light opacity-0 shadow-md transition-all duration-300 ease-out dark:border-surface-800 dark:bg-form-dark"
            :class="pickerOpen ? 'scale-100 opacity-100' : ''"
            style="position: absolute; z-index: 10"
        >
            <div class="flex flex-col">
                <!-- Header with Month and Year -->
                <div class="flex items-center justify-between">
                    <div
                        class="flex items-center justify-between gap-1 px-2 py-1 text-lg font-bold"
                    >
                        <div class="flex gap-1">
                            <span
                                class="cursor-pointer hover:text-primary-500"
                                @click="pickerView = 'months'"
                            >
                                {{ formatDate(selectedDate, 'MMMM') }}
                            </span>
                            <span
                                class="cursor-pointer hover:text-primary-500"
                                @click="pickerView = 'years'"
                            >
                                {{ formatDate(selectedDate, 'YYYY') }}
                            </span>
                        </div>
                    </div>
                    <!-- Navigation Buttons -->
                    <div class="flex items-center justify-between px-2 py-1">
                        <button
                            class="p-1 hover:text-primary-500"
                            type="button"
                            @click="offsetDate(-1)"
                        >
                            <!-- Left Arrow Icon -->
                            <svg
                                class="h-5 w-5"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    fill="currentColor"
                                    d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"
                                />
                            </svg>
                        </button>
                        <button
                            class="p-1 hover:text-primary-500"
                            type="button"
                            @click="offsetDate(1)"
                        >
                            <!-- Right Arrow Icon -->
                            <svg
                                class="h-5 w-5"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    fill="currentColor"
                                    d="M8.59 16.59L13.17 12 8.59 7.41 10 6l6 6-6 6z"
                                />
                            </svg>
                        </button>
                    </div>
                </div>

                <!-- Days View -->
                <div
                    class="flex flex-col items-center"
                    v-if="pickerView === 'days'"
                >
                    <!-- Weekdays Header -->
                    <div class="grid w-full grid-cols-7 gap-1 p-2">
                        <div
                            v-for="(weekday, ix) in weekdays"
                            :key="'weekday-' + ix"
                            class="text-center font-bold"
                        >
                            {{ weekday }}
                        </div>

                        <!-- Days Grid -->
                        <div
                            v-for="(day, ix) in daysInMonth"
                            :key="'day-' + ix"
                            class="cursor-default rounded p-1 text-center"
                            :class="[
                                day.currentMonth
                                    ? 'cursor-pointer text-ink-950 hover:bg-primary-100 hover:!text-ink-950 dark:text-ink-50'
                                    : 'text-ink-400 dark:text-ink-600',
                                isSelectedDay(day)
                                    ? 'bg-primary-500 !text-ink-50'
                                    : '',
                            ]"
                            @click="selectDate(day)"
                        >
                            {{ day.day }}
                        </div>
                    </div>
                    <hr
                        class="mx-2 w-full border-surface-100 dark:border-surface-800"
                    />
                    <!-- TimePicker Component -->
                    <template v-if="withTime">
                        <TimePicker
                            class="my-2"
                            v-model="selectedTime"
                            :hours24="true"
                        />
                    </template>
                </div>

                <!-- Months View -->
                <div
                    class="grid grid-cols-3 gap-1 p-2"
                    v-if="pickerView === 'months'"
                >
                    <div
                        v-for="ix in monthNames.map((_, index) => index)"
                        :key="'month-' + ix"
                        class="cursor-pointer rounded p-1 text-center"
                        :class="[
                            'hover:bg-primary-100 hover:!text-ink-950 dark:text-ink-50',
                            selectedDate.getMonth() === ix
                                ? 'bg-primary-500 !text-ink-50'
                                : '',
                        ]"
                        @click.stop="changeDateMonth(ix)"
                    >
                        {{ monthAbbreviations[ix] }}
                    </div>
                </div>

                <!-- Years View -->
                <div
                    class="grid grid-cols-3 gap-1 p-2"
                    v-if="pickerView === 'years'"
                >
                    <div
                        v-for="(year, ix) in getYearsArray()"
                        :key="'year-' + ix"
                        class="cursor-pointer rounded p-1 text-center"
                        :class="[
                            'hover:bg-primary-100 hover:!text-ink-950 dark:text-ink-50',
                            selectedDate.getFullYear() === year
                                ? 'bg-primary-500 !text-ink-50'
                                : '',
                        ]"
                        @click.stop="changeDateYear(year)"
                    >
                        {{ year }}
                    </div>
                </div>
            </div>
        </div>

        <!-- Date Picker Button -->
        <button
            type="button"
            ref="buttonRef"
            @click="toggleSelector"
            class="mt-4 cursor-default rounded border border-surface-300 bg-form-light px-3 py-1.5 text-center focus:outline-none focus:ring-1 focus:ring-primary-500 focus:ring-opacity-50 focus:ring-offset-2 focus:ring-offset-surface-50 dark:border-surface-800 dark:bg-form-dark dark:focus:ring-offset-surface-950"
        >
            {{ formatDate(selectedDate, props.format) }}
        </button>
    </div>
</template>
