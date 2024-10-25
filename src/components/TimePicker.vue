<script setup lang="ts">
const props = defineProps({
    modelValue: {
        type: String,
        default: '00:00',
    },
    hours24: {
        type: Boolean,
        default: true,
    },
})

const emit = defineEmits(['update:modelValue'])

const hoursValue = ref(0)
const minutesValue = ref(0)
const ampm = ref('AM')

const inputHour = ref('00')
const inputMinute = ref('00')

// Initialize values
parseModelValue(props.modelValue || '00:00')

function parseModelValue(value: string) {
    const time = value.trim() // Expected to be 'HH:MM' in 24-hour format
    const [hoursStr, minutesStr] = time.split(':')
    let hours = parseInt(hoursStr)
    const minutes = parseInt(minutesStr)

    if (isNaN(hours)) hours = 0
    if (isNaN(minutes)) minutesValue.value = 0
    else minutesValue.value = minutes

    if (!props.hours24) {
        // Convert hours to 12-hour format and set AM/PM
        if (hours === 0) {
            hoursValue.value = 12
            ampm.value = 'AM'
        } else if (hours < 12) {
            hoursValue.value = hours
            ampm.value = 'AM'
        } else if (hours === 12) {
            hoursValue.value = 12
            ampm.value = 'PM'
        } else {
            hoursValue.value = hours - 12
            ampm.value = 'PM'
        }
    } else {
        // 24-hour mode
        hoursValue.value = hours
        // ampm is not used in 24-hour mode
    }

    inputHour.value = hoursValue.value.toString().padStart(2, '0')
    inputMinute.value = minutesValue.value.toString().padStart(2, '0')
}

// Watch for changes in modelValue prop
watch(
    () => props.modelValue,
    (newVal) => {
        parseModelValue(newVal)
    }
)

// Hour input functions
function onHourInput(event: Event) {
    const value = (event.target as HTMLInputElement).value
    // Allow only digits
    inputHour.value = value.replace(/\D/g, '')
}

function onHourBlur() {
    let numericValue = parseInt(inputHour.value)
    if (isNaN(numericValue)) {
        numericValue = props.hours24 ? 0 : 12
    } else if (props.hours24) {
        numericValue = Math.max(0, Math.min(numericValue, 23))
    } else {
        numericValue = Math.max(1, Math.min(numericValue, 12))
    }
    hoursValue.value = numericValue
    inputHour.value = numericValue.toString().padStart(2, '0')
    emitModelValue()
}

function onHourKeydown(event: KeyboardEvent) {
    if (event.key === 'ArrowUp') {
        event.preventDefault()
        incrementHour()
    } else if (event.key === 'ArrowDown') {
        event.preventDefault()
        decrementHour()
    }
}

function incrementHour() {
    let newHour = hoursValue.value + 1
    if (props.hours24) {
        if (newHour > 23) newHour = 0
    } else {
        if (newHour > 12) {
            newHour = 1
        }
    }
    hoursValue.value = newHour
    inputHour.value = newHour.toString().padStart(2, '0')

    // Toggle AM/PM if needed
    if (!props.hours24 && hoursValue.value === 12) {
        toggleAmPm()
    }

    emitModelValue()
}

function decrementHour() {
    let newHour = hoursValue.value - 1
    if (props.hours24) {
        if (newHour < 0) newHour = 23
    } else {
        if (newHour < 1) {
            newHour = 12
        }
    }
    hoursValue.value = newHour
    inputHour.value = newHour.toString().padStart(2, '0')

    // Toggle AM/PM if needed
    if (!props.hours24 && hoursValue.value === 11) {
        toggleAmPm()
    }

    emitModelValue()
}

function toggleAmPm() {
    ampm.value = ampm.value.toUpperCase() === 'AM' ? 'PM' : 'AM'
    emitModelValue()
}

// Minute input functions
function onMinuteInput(event: Event) {
    const value = (event.target as HTMLInputElement).value
    // Allow only digits
    inputMinute.value = value.replace(/\D/g, '')
}

function onMinuteBlur() {
    let numericValue = parseInt(inputMinute.value)
    if (isNaN(numericValue)) {
        numericValue = 0
    } else {
        numericValue = Math.max(0, Math.min(numericValue, 59))
    }
    minutesValue.value = numericValue
    inputMinute.value = numericValue.toString().padStart(2, '0')
    emitModelValue()
}

function onMinuteKeydown(event: KeyboardEvent) {
    if (event.key === 'ArrowUp') {
        event.preventDefault()
        incrementMinute()
    } else if (event.key === 'ArrowDown') {
        event.preventDefault()
        decrementMinute()
    }
}

function incrementMinute() {
    let newMinute = minutesValue.value + 1
    if (newMinute > 59) {
        newMinute = 0
        incrementHour() // This will handle AM/PM toggle if needed
    }
    minutesValue.value = newMinute
    inputMinute.value = newMinute.toString().padStart(2, '0')
    emitModelValue()
}

function decrementMinute() {
    let newMinute = minutesValue.value - 1
    if (newMinute < 0) {
        newMinute = 59
        decrementHour() // This will handle AM/PM toggle if needed
    }
    minutesValue.value = newMinute
    inputMinute.value = newMinute.toString().padStart(2, '0')
    emitModelValue()
}

function emitModelValue() {
    let hour = hoursValue.value
    if (!props.hours24) {
        // Convert to 24-hour format for modelValue
        if (ampm.value.toUpperCase() === 'PM' && hour < 12) {
            hour += 12
        } else if (ampm.value.toUpperCase() === 'AM' && hour === 12) {
            hour = 0
        }
    }
    const hourStr = hour.toString().padStart(2, '0')
    const minuteStr = minutesValue.value.toString().padStart(2, '0')
    // Always emit modelValue in 24-hour format without AM/PM
    emit('update:modelValue', `${hourStr}:${minuteStr}`)
}
</script>

<template>
    <div class="inline-flex items-center gap-1">
        <div class="flex gap-0.5">
            <input
                type="text"
                v-model="inputHour"
                @input="onHourInput"
                @blur="onHourBlur"
                @keydown="onHourKeydown"
                maxlength="2"
            />
            <div class="flex flex-col justify-between">
                <button
                    @click="incrementHour"
                    class="arrow-button"
                    tabindex="-1"
                >
                    <svg
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-1.5 w-1.5"
                        fill="currentColor"
                    >
                        <path d="m12 3.167 10.406 17.666H1.595L12 3.167Z" />
                    </svg>
                </button>
                <button
                    @click="decrementHour"
                    class="arrow-button"
                    tabindex="-1"
                >
                    <svg
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-1.5 w-1.5"
                        fill="currentColor"
                    >
                        <path d="M12 20.833 22.406 3.167H1.595L12 20.833Z" />
                    </svg>
                </button>
            </div>
        </div>
        <span>:</span>
        <div class="5 flex gap-0">
            <input
                type="text"
                v-model="inputMinute"
                @input="onMinuteInput"
                @blur="onMinuteBlur"
                @keydown="onMinuteKeydown"
                maxlength="2"
            />
            <div class="flex flex-col justify-between">
                <button
                    @click="incrementMinute"
                    class="arrow-button"
                    tabindex="-1"
                >
                    <svg
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-1.5 w-1.5"
                        fill="currentColor"
                    >
                        <path d="m12 3.167 10.406 17.666H1.595L12 3.167Z" />
                    </svg>
                </button>
                <button
                    @click="decrementMinute"
                    class="arrow-button"
                    tabindex="-1"
                >
                    <svg
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-1.5 w-1.5"
                        fill="currentColor"
                    >
                        <path d="M12 20.833 22.406 3.167H1.595L12 20.833Z" />
                    </svg>
                </button>
            </div>
        </div>
        <template v-if="!props.hours24">
            <button @click="toggleAmPm" class="ampm-button">
                {{ ampm }}
            </button>
        </template>
    </div>
</template>

<style scoped>
input {
    @apply w-12 rounded border border-surface-300 bg-surface-100 px-2 py-1 text-center focus:outline-none focus:ring-1 focus:ring-primary-500 focus:ring-opacity-50 focus:ring-offset-2 focus:ring-offset-surface-50 dark:border-surface-800 dark:bg-surface-800 dark:focus:ring-offset-surface-950;
}

.ampm-button {
    @apply ml-2 cursor-pointer rounded border border-surface-300 bg-surface-100 px-2 py-1 focus:outline-none focus:ring-0 dark:border-surface-800 dark:bg-surface-800;
}

.arrow-button {
    @apply cursor-pointer rounded border border-surface-300 bg-surface-100 px-2 py-1 focus:outline-none focus:ring-0 dark:border-surface-800 dark:bg-surface-800;
}
</style>
